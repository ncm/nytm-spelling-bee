```puzzlegen.cc``` is a simple program in C++14 that generates anagram
puzzles as found in the New York Times Magazine, that they call
"Spelling Bee".  These puzzles present a circle of six letters
around a seventh, central letter, like
```
   M O
  P I S
   T U
```
The goal is to find common words that use only the letters in the
set, and that all use the central letter.  Words that use all the
letters score extra: one point for each lesser word, and three for
each that uses all seven.  For example, for the letters above,
"mitosis" scores 1, "optimums" 3.  The program only emits puzzles
that that it finds have between 25 and 33 points possible, given
the words in its list.  Typically one should be satisfied to find
20 points' worth.

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

```solve.sh``` is a simpler program that, given such a puzzle, lists
words found in /usr/share/dict/words that solve the puzzle. An
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

```puzzlegen.cc``` is probably more interesting as an example of optimized
modern C++ coding than as a generator of puzzles.  It uses bits in
a 32-bit word to represent sets of letters, bitwise arithmetic to
step through the set and qualify words, lambda functions (one with
an ```auto argument```, equivalent to a template) to compose operations,
STL algorithms, new-style for-loops over containers, and a compiler
intrinsic ```__builtin_popcount``` to generate a single-instruction count
of nonzero bits in a machine word.

As important is what it doesn't use.  It doesn't store the actual
words it reads, as they are not useful.  It uses ```<set>```, not
```<unordered_set>```, because set is only 10% slower but produces
more-pleasingly ordered output.  It makes only one pass through all
the candidate words for each candidate letter-set.  It discards words
on input that cannot be solutions.

It does depend on a runtime character set with contiguous alpha characters,
a compiler with the aforementioned ```__builtin_popcount``` extension, and
a ```/usr/share/dict/words``` file in the right place.
