```puzzlegen``` is a simple program that generates all
possible versions of an anagram puzzle as found in the New York Times
Magazine, that they call "Spelling Bee".  These puzzles present a circle
of six letters around a seventh, central letter, like
```
    M   O
  P   I   S
    T   U
```
The goal of the puzzle is to find words that use only the letters in the
set, and that all use the central letter.  Words that use all the letters
score extra: one point for each lesser word, and three for each that uses
all seven.  For example, for the letters above, "mitosis" scores 1,
"optimums" 3.  The program only emits puzzles that that it finds have
between 26 and 32 points possible, given the words in its list.
Typically one should be satisfied to find 20 points' worth.

Output is a list of seven-letter sets, like
```
  $ ./puzzlegen
  ...
  imOprSy
  imOprtu
  Imopstu
  ImoPstv
```
Capital letters in output are candidates for the central letter.

```solve.sh``` is a much simpler program that, given such a puzzle,
lists words found in /usr/share/dict/words that solve the puzzle. An
excerpt from its output for the puzzle above is,
```
  $ ./solve.sh imopstu
  ...
  optimum
  optimums *
  osmosis
  pimps
```
with three-point words suffixed " *".  (The central letter comes first
in the command-line argument.)

### Internals

```puzzlegen``` may be more interesting as an example of optimized modern
C++14 and Rust coding than as a generator of puzzles.  In C++, it uses bits in
a 32-bit word, via bitset<>, to represent sets of letters, bitwise arithmetic
to step through the set and qualify words, and new-style for-loops over
containers.  The Rust version does almost precisely the same operations,
but in a functional style that turns out to run a little faster than if
transcribed straight from the C++.

As important is what it doesn't use.  It doesn't store the actual words it
reads, as they are not useful.  It uses ```<map>```, not ```<unordered_map>```,
because (a) with ```map``` it is *exactly* as fast, but (b) produces more-
pleasingly ordered output.  It makes only one pass through all the candidate
words for each candidate letter-set.  It discards words on input that cannot
be solutions.

It does depend on a runtime character set with contiguous alphabetic
characters, and, by default, a ```/usr/share/dict/words``` file in the right
place.

The C++ version puzzlegen-old.cc, built with gcc-5, runs faster on Intel
Haswell than other C++ versions, while all the C++ versions run about the
same speed on Westmere.  I.e., on Haswell most versions run artifically
slowly.

The Rust version runs almost exactly as fast as the  C++ version.

  - puzzlegen.cc     -- fast C++ version
  - puzzlegen.rs     -- in Rust, reading byte-by-byte

Alternative versions of the programs, found in the variants directory, differ:

  - puzzlegen-bitset.cc -- uses local bitset_set.h iterators
  - puzzlegen-int.cc -- uses unsigned int rather than std::bitset<26>
  - puzzlegen-old.cc -- posted in gcc bug #67153, only version fast on Haswell
  - puzzlegen-str.cc -- reads words into std::string
  - puzzlegen-str.rs -- similarly, reading into a Vec<u8>
  
