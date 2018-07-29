```puzzlegen``` is a short program that generates, in as little as 60 ms,
all possible versions of an anagram puzzle by Frank Longo called "Spelling Bee", 
as found in the Sunday New York Times Magazine.  These puzzles present a 
circle of six letters ranged around a seventh, central letter, like
```
    M   O
  P   I   S
    T   U
```
The goal of the puzzle is to find words of five or more letters that use 
only the letters in the set, and that all use the central letter.  Words 
that use all seven letters score three points, the rest one.  For example, 
for the letters above, "mitosis" scores 1, "optimums" 3.  

The program as presented only emits puzzles that that it finds have between 
26 and 32 points possible, given the words in its list. Typically one is 
advised to be satisfied to find 20 points' worth, but there is no reason to 
limit yourself to that.

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

```solve.sh```, also here, is a much simpler script that, given such a  
puzzle, lists words found in /usr/share/dict/words that solve the puzzle. 
An excerpt from its output for the puzzle above is,
```
  $ ./solve.sh imopstu
  ...
  optimum
  optimums *
  osmosis
  pimps
  ...
  28
```
with three-point words suffixed " *".  (The central letter comes first
in the command-line argument.)

### Internals

```puzzlegen``` is perhaps more interesting as an example of optimized modern
C++ and Rust coding, than as a generator of puzzles.  In C++, it uses bits in
a 32-bit word, via bitset<>, to represent sets of letters, bitwise arithmetic
to step through the set and qualify words, and new-style for-loops over
containers.  The Rust version does almost precisely the same operations,
but in a functional style that turns out to run a little faster than if
transcribed straight from the C++, but finally equally as fast as the C++.

As important is what it doesn't use.  It doesn't store the actual words it
reads, as they are not useful.  It makes only one pass through all the 
candidate words for each candidate letter-set.  It discards words on input 
that cannot be solutions.

It does depend on a runtime character set with contiguous alphabetic
characters, and, by default, a ```/usr/share/dict/words``` file in the 
right place.

The variation puzzlegen-old.cc, built with gcc-8, runs faster on Intel
Haswell than other variations, while all the C++ variations run about the
same speed on Westmere.  I.e., on Haswell most versions run artifically
slowly.

  - puzzlegen.cc     -- fast C++ version
  - puzzlegen.rs     -- in Rust
  - article.md       -- pandoc article about comparison

Alternative versions of the programs, found in the variants directory, differ:

  - puzzlegen-bitset.cc -- uses local bitset_set.h iterators
  - puzzlegen-int.cc -- uses unsigned int rather than std::bitset<26>
  - puzzlegen-old.cc -- posted in gcc bug #67153, only version fast on Haswell
  - puzzlegen-str.cc -- reads words into std::string
  - puzzlegen-str.rs -- similarly, reading into a Vec<u8>
 
