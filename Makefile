CXX = g++
RUSTC = rustc
STDLIB =
CXXFLAGS = -O3 $(STDLIB) -Wall -march=native -mtune=native
RUSTFLAGS = -C opt-level=3 -C target-cpu=native

run: puzzlegen
	$(RTLIBPATH) time ./puzzlegen | tee out | wc -l
	cmp out.ref out

runrs: puzzlegen-rust
	time ./puzzlegen-rust | tee out | wc -l
	cmp out.ref out

puzzlegen: puzzlegen.cc bitset_set.h
	$(CXX) $(CXXFLAGS) -g3 -std=c++14 $< -o $@

puzzlegen-rust: puzzlegen.rs
	$(RUSTC) $(RUSTFLAGS) -g $< -o $@
