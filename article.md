
# Rust vs. C++: Fine-grained Performance
| Reply to: Nathan Myers `<ncm@cantrip.org>`
| Git: <http://github.com/ncm/nytm-spelling-bee>
| Reddit: <https://redd.it/42qc78>
| Published: 2016-01-25
| Last Edit: 2016-02-03

If Rust is to take on work previously reserved to C++, we need to know
how well it does what C++ does best. What's fast, what's slow? What's
harder to do, what's easier?  I wouldn't know how to look up answers for
those questions, but I can write programs.

I had a C++ program that was just the right length to experiment with --
one printed page -- and that did nothing tricky to express in an unfamilar
language. (It generates all possible versions of a puzzle called
"Spelling Bee" found in the *New York Times Magazine*.) I began by
transcribing the program straight across to equivalent Rust code.
The Rust program turned out close to the same length, but only half
as fast. As I made the Rust code more idiomatic, it got faster. At the
same time, I worked to speed up the C++ program, still hewing to the
original one-page limit. After each change, I checked performance.
Few programs get this much attention to optimization.

The C++ version now runs four times as fast as when I started; about as
fast, I think, as it can be made without making it longer, or parallel,
or using third-party libraries. In 75 ms on modern hardware, it performs
some 190 million basic operations (at a cycle per iteration (!)), filtering
to 5 million more-complex operations (at under 16 cycles per). Meanwhile,
the Rust program does about the same operations in about the same time:
a percent or two faster or slower on various hardware. Many variations
that seemed like they ought to run the same speed or faster turned out
slower.

Below, I present each program in fragments. The code may be denser than
you are used to, just to keep it to one printed page. When I write "much
slower", below, it might mean 1.3x to 2x, not the order of magnitude it
might mean outside systems programming.  Huge swaths of both languages
are ignored: C++ templates, destructors, futures, lambdas; Rust channels,
threads, traits, cells, lifetimes, borrowing, modules, macros. Those are
essential to really using the language, but this project is deliberately
about the nitty-gritty of coding.

So, the programs. First, dependencies: headers, modules.

C++:

```cpp
#include <iostream>
#include <fstream>
#include <vector>
#include <iterator>
#include <string>
#include <algorithm>
#include <bitset>
```

And Rust:

```rust
use std::io::prelude::*;
use std::{fs, io, env, process};
```

Rust wins, here.  Rust provides much of its standard library by default,
or with just the first line above, and supports ganging module `use`s
on one line.  Rust's module system is exemplary; any future language
that ignores its lessons courts failure.

Next, argument and input-file processing.

C++:

```cpp
int main(int argc, char** argv) {
    std::string name = (argc > 1) ? argv[1] : "/usr/share/dict/words";
    std::ifstream fs;
    std::istream& file = name == "-" ? std::cin : (fs.open(name), fs);
    if (!file)
        return std::cerr << "file open failed: \"" << name << "\"\n", 1;
```

And Rust:

```rust
fn main() {
    let fname = &*env::args().nth(1).unwrap_or("/usr/share/dict/words".into());
    let stdin = io::stdin();
    let file: Box<Read> = match fname {
        "-" => Box::new(stdin.lock()),
        _ => Box::new(fs::File::open(fname).unwrap_or_else(|err| {
                 writeln!(io::stderr(), "{}: \"{}\"", err, fname).unwrap();
                 process::exit(1);
             }))
    };
```

Point, C++. This stuff just takes more code to do in Rust.  Most people
coding Rust would let a toy program like this report usage errors by
"panicking", although that produces very ugly output.  Rust does make
it hard to accidentally ignore an I/O error result, which is good so
long as people don't get used to deliberately ignoring them; but in
Rust, ignoring errors takes more work.

Both programs take an optional file name on the command line, and can
read from `stdin`, which is convenient for testing. On standard Linux
systems the "words" file is a list of practical English words, including
proper names, contractions, and short words that must be filtered out.
Notable, here, is the use of `Box` for type-erasure so `io::stdin` can be
substituted for the `fs::File` handle. The odd construct `&*` extracts
the character sequence hidden inside the (`Option`-wrapped) `String`
produced by `nth()`, so that `match` will have something it can compare
directly to the built-in literal string `"-"`.

I don't mind locking `io::stdin`, to get faster input, but requiring
that the call to `lock()` be in a separate statement is weird.

Data structure and input setup follows, along with the input loop header.

C++:

```cpp
    std::vector<unsigned> words; words.reserve(1<<15);
    std::vector<unsigned> sevens; sevens.reserve(1<<14);
    std::bitset<32> word; int len = 0;
    for (std::istreambuf_iterator<char> in(file), eof; in != eof; ++in) {
```

Rust:

```rust
    let mut words = Vec::with_capacity(1 << 15);
    let mut sevens = Vec::with_capacity(1 << 14);
    let (mut word, mut len) = (0u32, 0);
    for c in io::BufReader::new(file).bytes().filter_map(Result::ok) {
```

One point here for Rust. Rust integer types support `count_ones()`. The
C++ version needs `std::bitset` for its member `count()` (which would be
`size()` if `bitset` were a proper C++ set) because it is the only way in
C++ to get at the `POPCNT` instruction without using a non-standard
compiler intrinsic like Gcc's `__builtin_popcountl`. Using `bitset<32>`
instead of `<26>` suppresses some redundant masking operations. Since the
smallest `bitset<>` on Gcc/amd64 is 64 bits, the values are stored more
efficiently as `unsigned`.  Rust has no equivalent to bitset (yet), so
we're lucky all the bits we needed fit in an available integer type;
but similarly so for C++.

The actual types of the Rust `sevens` and `words` vectors are deduced from
the way they are used way further down in the program.  The `filter_map`
call strips off a `Result` wrapping, discarding any file-reading errors.

Next, we have the the input state machine.

C++:

```cpp
        if (*in == '\n') {
            if (len >= 5 && ones <= 7)
                (ones < 7 ? words : sevens).push_back(word.to_ulong());
            word = len = ones = 0;
        } else if (ones != 8 && *in >= 'a' && *in <= 'z') {
            ++len, ones = word.set(25 - (*in - 'a')).count();
        } else { ones = 8; }
```

And Rust:

```rust
        if c == b'\n' {
            if len >= 5 && ones <= 7
                { if ones == 7 { sevens.push(word) } else { words.push(word) } }
            word = 0; len = 0; ones = 0;
        } else if ones != 8 && c >= b'a' && c <= b'z' {
            word |= 1 << (25 - (c - b'a')); len += 1; ones = word.count_ones()
        } else { ones = 8 }
```

These are exactly even. The state machine is straightforward: gather up
and store eligible words, and skip past ineligible words. On earlier
versions of the Rust compiler, I had to use an iterator pipeline, using
`.scan()`, `match`, `.filter()`, and `.collect()`, at twice the line count,
to get tolerable performance. Now the loop is faster. A `match` would work
here, but the code would be longer.  Rust could have just one `push` call,
as in the C++ version, but it would be ugly, and slower besides.

Incidentally, I don't know why I can write
```
    let (mut word, mut len, mut skip) = (0u32, 0, false);
```
but not
```
    (word, len, skip) = (0, 0, false);
```
Obviously the present syntax doesn't allow it, but syntax is not physics.
Surprising syntactic restrictions make the language more complex for users.

Next, we need to sort the collection of seven-different-letter words,
and count duplicates.

C++:

```cpp
    std::sort(sevens.begin(), sevens.end());
    std::vector<unsigned> counts(sevens.size());
    int count = -1; unsigned prev = 0;
    for (auto seven : sevens) {
        if (prev != seven)
            sevens[++count] = prev = seven;
        counts[count] += 3;
    }
```

And Rust:

```rust
    sevens.sort();
    let (mut count, mut prev, mut counts) = (!0, 0, vec![0; sevens.len()]);
    for i in 0..sevens.len() {
        if prev != sevens[i]
            { count += 1; prev = sevens[i]; sevens[count] = prev; }
        counts[count] += 3;
    }
```

These are very close to even. In Rust, when working with two elements
of the same vector, indexing is more comfortable, because it avoids
ownership conflicts.  One hopes that the optimizer can see that `count`
cannot exceed `sevens.len()`, so that bounds checking may be elided.
Rust doesn't like indexing with a signed integer, so we start the index
at all ones and let it roll over to 0, instead.

The program to this point is all setup, accounting for a small fraction
of run time. Using `<map>` or `BTreeMap`, respectively, would make this
last fragment unnecessary, in exchange for 3% more total run time.

Rust's convenience operations for booleans, by the way, are curiously
neglected, vs. `Result` and `Option`.  For example, some code would read
better if I could write something like:
```rust
    return is(c).then_some(||f(c))
```
instead of
```rust
    return is(c) { Some(f(c)) } else { None }
```
The body of `then_some()` is just a one-liner, but to be useful it needs
to be standard.^[I do not dare to propose "`ergo_some()`".]

The main loop is presented below, in two phases.  The first half is where
the program spends practically all its time.

C++:

```cpp
    for (; count >= 0; --count) {
        unsigned const seven = sevens[count];
        int bits[7], int scores[7];
        for (unsigned rest = seven, place = 7; place-- != 0; rest &= rest - 1) {
            bits[place] = std::bitset<32>((rest & ~(rest - 1)) - 1).count();
            scores[place] = counts[count];
        }
        for (unsigned word : words)
            if (!(word & ~seven))
                for (int place = 0; place < 7; ++place)
                    scores[place] += (word >> bits[place]) & 1;
```

And Rust:

```rust
    let stdout = io::stdout();
    let mut sink = io::BufWriter::new(stdout.lock());
    for count in (0..(count + 1)).rev() {
        let seven = sevens[count];
        let (mut rest, mut bits) = (seven, [0;7]);
        for place in (0..7).rev()
            { bits[place] = rest.trailing_zeros(); rest &= rest - 1 }
        let scores = words.iter()
            .filter(|&word| word & !seven == 0)
            .fold([counts[count];7], |mut scores, &word| {
                for place in 0..7
                     { scores[place] += (word >> bits[place]) & 1; }
                scores
            });
```

This is close to even. Again, the first two lines in the Rust code seem
excessive just to get faster output.

The first inner loop explodes the positions of bits in `seven` out to
the `bits` array, one per element, so that subsequent loops can be
unrolled and executed out-of-order. (Optimizers actually seem able to
do this all by themselves, but the code is shorter this way, and maybe
easier to understand.)  Rust's `trailing_zeros()` maps to the machine
instruction `CTZ`.  C++ offers no direct equivalent, but
`bitset<>::count()` serves.

The "`.filter`" line is executed 190M times.  Only some 720K
iterations reach the "`.fold()`", but the innermost loop runs 5M
times, and `*score` is actually incremented 3M times.  That loop is
where the program spends more time than anywhere else.  The "`fold()`",
with its `scores` state passed along from one iteration to the next,
is much faster than the equivalent loop with outer-scope state
variables.  The `words` iterator is "lazy", but the "`fold()`" call
drives the it to completion.

I found that iterating over a array with (e.g.) "`array.iter()`" was much
faster than with "`&array`", although it should be the same. I suppose
that will be fixed someday. Curiously, changing `scores` to an array of
16-bit values slows down earlier versions of the C++ program by quite a
large amount -- almost 10% in some tests -- as the compiler yields to
temptation and puts `scores` in an XMM register. The Rust program was
also affected, but less so.

The second phase of the main loop does output based on the scores
accumulated above.

C++:

```cpp
        bool any = false;
        char out[8];
        for (int place = 0; place != 7; ++place) {
            int points = scores[place];
            char a = (points >= 26 && points <= 32) ? any = true, 'A' : 'a';
            out[place] = a + (25 - bits[place]);
        }
        if (any)
            out[7] = '\n', std::cout.rdbuf()->sputn(out, 8);
    }
}
```

And Rust:

```rust
        let (mut any, mut out) = (false, *b".......\n");
        for place in 0..7 {
            let a = match scores[place]
               { 26 ... 32 => { any = true; b'A' }, _ => b'a' };
            out[place] = a + (25 - bits[place]) as u8
        }
        if any
            { sink.write(&out).unwrap(); };
    }
}
```

I call this about even, too.

The loop walks the `out` array, pairing each byte with its
corresponding score and a bit position from `bits`. The output is built
of `u8` bytes instead of proper Rust characters because operations on
character and string types are slowed by runtime error checking and
conversions.  (The algorithm used here only works with ASCII anyhow.)
Unlike in the C++ code, the `out` elements are initialized twice
(although it's possible the optimizer elides it).  People complain
online about the few choices available for initializing arrays, which
often requires the arrays to be made unnecessarily mutable.

Curiously, most variations of the C++ version run only half as fast as
they should on Intel Haswell chips, probably because of branch prediction
failures^[<https://gcc.gnu.org/bugzilla/show_bug.cgi?id=67153>].
(Wrapping "`!(word & ~seven)`" in `__builtin_expect(..., false)` works
around the hardware bug.) It's possible that Gcc will learn someday to
step around the Haswell bug by itself, or new microcode will fix it,
but I'm amazed that Intel released Haswell that way.^[Maybe I shouldn't
be: <http://danluu.com/cpu-bugs/>]  I don't know yet if it Intel fixed
it in Broadwell or Skylake.

Rust has some rough edges, but coding in it was kind of fun. As with
C++, if a Rust program compiles at all, it generally works, more or
less, but perhaps more. Rust's support for generics is improving, but
is still well short of what a Rusty STL would need. The compiler was
slow, but they're working hard on that, and I believe its speed will
be unremarkable by this time next year. (I could forgive its slowness
if it kept its opinions on redundant parentheses to itself.)  Rust's
iterator primitives string together nicely.

It is a signal achievement to match C++ in low-level performance and
brevity while surpassing it in safety, with reasonable prospects to match
its expressive power in the foreseeable future. C++ is a rapidly moving
target, held back only by legacy compatibility requirements, so Rust
will need to keep moving fast just to keep up.  While Rust could "jump
the shark" any time, thus far there's every reason to expect to see,
ten years on, recruiters advertising for warm bodies with ten years'
production experience coding Rust.

[Thanks to Steve Klabnik, `eddyb`, `leonardo`, `huon`, `comex`,
`marcianix`, and `alexeiz` for major improvements to the code and
to the article. The mistakes remain mine, all mine. Material alterations:

    a. Examples for `then_some` improved
    b. In C++, s/short/int/; Rust s/0u16/0/; resulting in speedup
    c. Simplify output loop -- rustc has improved, allowing simpler code
    d. Simplify argument processing, slightly
    e. Improve counting logic
    f. Enable unrolled/out-of-order loops by precomputing bit positions
    g. Replace innermost-loop conditional branch with a bitwise operation
    h. Improve state machine test for valid word characters
]
