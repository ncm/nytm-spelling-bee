
# Rust vs. C++: Fine-grained Performance
| Reply to: Nathan Myers `<ncm@cantrip.org>`
| Git: <http://github.com/ncm/nytm-spelling-bee>
| Reddit: <https://redd.it/42qc78>
| Published: 2016-01-25
| Last Edit: 2016-01-26

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
    let fname = env::args().nth(1).unwrap_or("/usr/share/dict/words".into());
    let stdin = io::stdin();
    let file: Box<Read> = match &fname[..] {
        "-" => Box::new(stdin.lock()),
        _ => Box::new(fs::File::open(&name).unwrap_or_else(|err| {
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
substituted for the `fs::File` handle. The odd construct `&fname[..]`
is needed because `"-"` is a built-in character sequence, but `fname`
is a library `String` type with a character sequence hidden inside;
`match` needs help to see it.

I don't mind locking `io::stdin`, to get faster input, but requiring
that the call to `lock()` be in a separate statement is weird.

Data structure and input setup follows, along with the input loop header.

C++:

```cpp
    std::vector<unsigned> words; words.reserve(1<<15);
    std::vector<std::pair<unsigned,short>> sevens; sevens.reserve(1<<15);
    std::bitset<32> word; int len = 0; bool skip = false;
    for (std::istreambuf_iterator<char> in(file), eof; in != eof; ++in) {
```

Rust:

```rust
    let mut words = Vec::with_capacity(1 << 15);
    let mut sevens = Vec::with_capacity(1 << 16);
    let (mut word, mut len, mut skip) = (0u32, 0, false);
    for c in io::BufReader::new(file).bytes().filter_map(Result::ok) {
```

One point here for Rust. Rust integer types support `count_ones()`. The
C++ version needs `std::bitset` for its member `count()` (which would be
`size()` if `bitset` were a proper C++ set) because it is the only way in
C++ to get at the `POPCNT` instruction without using a non-standard
compiler intrinsic like Gcc's `__builtin_popcountl`. Using `bitset<32>`
instead of `<26>` suppresses some redundant masking operations.  C++
`bitset` doesn't have an `operator<` (yet), so the bitsets are actually
stored as regular `unsigned`. (Since the smallest `bitset<>` on Gcc/amd64
is 64 bits, storing `unsigned` is more efficient anyway.)  Rust has no
equivalent to bitset (yet), so we're lucky we needed just 26 bits.

The actual types of the Rust `sevens` and `words` vectors are deduced from
the way they are used way further down in the program.  The `filter_map`
call strips off a `Result` wrapping, discarding any file-reading errors.

Next, we have the the input state machine.

C++:

```cpp
        if (*in == '\n') {
            if (!skip && len >= 5) {
                if (word.count() == 7) {
                    sevens.emplace_back(word.to_ulong(), 0);
                } else words.push_back(word.to_ulong());
            }
            word = len = skip = false;
        } else if (!skip && *in >= 'a' && *in <= 'z') {
            word.set(25 - (*in - 'a'));
            if (word.count() <= 7) ++len; else skip = true;
        } else { skip = true; }
    }
```

And Rust:

```rust
        if c == b'\n' {
            if !skip && len >= 5 {
                if word.count_ones() == 7 {
                        sevens.push((word, 0))
                } else { words.push(word) }
            }
            word = 0; len = 0; skip = false;
        } else if !skip && c >= b'a' && c <= b'z' {
            word |= 1 << (25 - (c - b'a'));
            if word.count_ones() <= 7 { len += 1 } else { skip = true }
        } else { skip = true }
    }
```

These are exactly even. The state machine is straightforward: gather up
and store eligible words, and skip past ineligible words. On earlier
versions of the Rust compiler, I had to use an iterator pipeline, using
`.scan()`, `match`, `.filter()`, and `.collect()`, at twice the line count,
to get tolerable performance. A `match` would work here, but the code
would be longer.

Incidentally, I don't know why I can write
```
    let (mut word, mut len, mut skip) = (0u32, 0, false);
```
but not
```
    (word, len, skip) = (0, 0, false);
```
Obviously the present syntax doesn't allow it, but syntax is not physics,
syntax is something we create in service of usefulness.  Surprising
syntactic restrictions make the language more complex for users.

Next, we need to sort the collection of seven-different-letter words,
and count duplicates.

C++:

```cpp
    std::sort(sevens.begin(), sevens.end(),
        [](auto a, auto b) { return a.first > b.first; });
    size_t place = 0;
    for (auto pair : sevens)
        if (pair.first != sevens[place].first)
            pair.second = 1, sevens[++place] = pair;
        else sevens[place].second++;
    sevens.resize(place + 1);
```

And Rust:

```rust
    sevens.sort_by(|a, b| b.0.cmp(&a.0));
    let mut place = 0;
    for i in 0..sevens.len() {
        if sevens[i].0 != sevens[place].0
            { place += 1; sevens[place] = sevens[i]; }
        sevens[place].1 += 1;
    }
    sevens.resize(place + 1, (0,0));
```

These are very close to even. In Rust, when working with two elements
of the same vector, indexing is more comfortable.  One hopes that the
optimizer can see that `place` cannot exceed `i`, so no bounds checking
is needed.

The program to this point is all setup, accounting for a small fraction
of run time. Using `<map>` or `BTreeMap`, respectively, would make this
last fragment unnecessary, in exchange for 3% more total run time.

Rust's convenience operations for booleans, by the way, are curiously
neglected, vs. `Result` and `Option`.  For example, some code would read
better if I could write something like:
```rust
    return is(c).then_some(f(c))
```
instead of
```rust
    return is(c) { Some(f(c)) } else { None }
```
The body of `then_some` is just a one-liner, but to be useful it needs
to be standard.

The main loop is presented below, in two phases.  The first half is where
the program spends most of its time.

C++:

```cpp
   for (auto sevencount : sevens) {
        unsigned const seven = sevencount.first;
        short scores[7] = { 0, };
        for (unsigned word : words)
            if (!(word & ~seven)) {
                unsigned rest = seven;
                for (int place = 7; --place >= 0; rest &= rest - 1)
                    if (word & rest & -rest)
                        ++scores[place];
            }
```

And Rust:

```rust
    let stdout = io::stdout();
    let mut sink = io::BufWriter::new(stdout.lock());
    for (&seven, &count) in sevens.iter() {
        let scores = words.iter()
            .filter(|&word| word & !seven == 0)
            .fold([0u16;7], |mut scores, word| {
                scores.iter_mut().fold(seven, |rest, score| {
                   if word & rest & !(rest - 1) != 0
                       { *score += 1 }
                   rest & rest - 1
                });
                scores
            });
```

This is close to even.

Again, the first two lines in the Rust code seem excessive just to get
faster output. The "`.filter`" line is executed 190M times. Only some
720K iterations reach the outer "`.fold()`", but the inner loop runs
5M times and `*score` is incremented 3M times.  That loop is where the
program spends more time than anywhere else.  The "`fold()`" with its
`scores` state passed along from one iteration to the next is much faster
than the equivalent loop with outer-scope state variables.  The two
nested "`fold()`" calls, as with "`collect()`" above, drive the lazy
iterators to completion.

The second phase does output based on the scores accumulated above.

C++:

```cpp
        bool any = false;
        unsigned rest = seven;
        char buf[8]; buf[7] = '\n';
        for (int place = 7; --place >= 0; rest &= rest - 1) {
            int points = scores[place] + 3 * sevencount.second;
            char a = (points >= 26 && points <= 32) ? any = true, 'A' : 'a';
            buf[place] = a + (25 - std::bitset<32>(~rest & (rest - 1)).count());
        }
        if (any)
            std::cout.rdbuf()->sputn(buf, sizeof(buf));
    }
}
```

And Rust:

```rust
        let mut out = *b".......\n";
        let (any, _) = scores.iter().zip(out.iter_mut().rev().skip(1))
            .fold((false, seven), |(mut any, rest), (&score, outc)| {
                let a = match score + 3 * count
                    { 26 ... 32 => { any = true; b'A' }, _ => b'a' };
                *outc = a + (25 - rest.trailing_zeros()) as u8;
                (any, rest & rest - 1)
            });
        if any
            { sink.write(&out).unwrap(); };
    }
}
```

I call this about even, too.

Rust's `trailing_zeros()` maps to the instruction `CTZ`.  C++ offers
no direct equivalent, but `bitset<>::count()` serves. I found that
iterating over a small array in Rust with (e.g.) "`array.iter()`" was
much faster than with "`&array`", although it should be the same. 
I suppose that will be fixed someday. As before, using outer-scope
variables can be much slower than passing `fold`'s state along to its
next iteration.

The `fold` walks the `out` array backward, skipping the newline, and
pairing each byte with a corresponding score and a one-bit in `seven`.
The output is built of `u8` bytes instead of proper Rust characters
because operating on character and string types would be slowed by
runtime error checking and conversions.  (The algorithm used here only
works with ASCII anyhow.)  Unlike in the C++ code, the `out` elements
are initialized twice. People complain online about the few choices
available for initializing arrays, which often requires the arrays
to be made unnecessarily mutable.

Curiously, most variations of the C++ version run only half as fast as
they should on Intel Haswell chips, probably because of branch prediction
failures^[<https://gcc.gnu.org/bugzilla/show_bug.cgi?id=67153>].
(Wrapping "`!(word & ~seven)`" in `__builtin_expect(..., false)` works
around the bug.) It's possible that Gcc will learn someday to step
around the Haswell bug by itself, or new microcode will fix it, but
I'm amazed that Intel released Haswell that way.^[Maybe I shouldn't
be: <http://danluu.com/cpu-bugs/>]  I don't know yet if
it's fixed in Broadwell or Skylake.

Rust has some rough edges, but coding in it was kind of fun. As with
C++, if a Rust program compiles at all, it generally works, more or
less. Rust's support for generics is improving, but is still well short
of what a Rusty STL would need. The compiler was slow, but they're
working hard on that, and I believe its speed will be unremarkable by
this time next year. (I could forgive its slowness if it kept its
opinions on redundant parentheses to itself.)  Rust's iterator 
primitives string together nicely.

It is a signal achievement to match C++ in low-level performance and
brevity while surpassing it in safety, with reasonable prospects to match
its expressive power in the near future. C++ is a rapidly moving target,
held back only by legacy compatibility requirements, so Rust will need
to keep moving fast just to keep up.  While Rust could "jump the shark"
any time, thus far there's every reason to expect to see, ten years
on, recruiters advertising for warm bodies with ten years' production
experience coding Rust.

[Thanks to Steve Klabnik, `eddyb`, `leonardo`, `huon`, `comex`, and
`marcianix` for major improvements to the code and to the article.
The mistakes remain mine, all mine.]

