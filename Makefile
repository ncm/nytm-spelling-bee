CXX = g++
RUSTC = rustc
STDLIB =
CXXFLAGS = -O3 $(STDLIB) -Wall -march=corei7 # -msse3 -mbmi -mbm2
RUSTFLAGS = -C opt-level=2 -C target-cpu=corei7 # -C target-feature="+sse4.2,+popcnt"

run: puzzlegen
	$(RTLIBPATH) time ./puzzlegen | tee out | wc -l
	cmp out.ref out

runrs: puzzlegen-rust
	time ./puzzlegen-rust | tee out | wc -l
	cmp out.ref out

puzzlegen: puzzlegen.cc # bitset_set.h
	$(CXX) $(CXXFLAGS) -g3 -std=c++14 $< -o $@

puzzlegen-rust: puzzlegen.rs
	$(RUSTC) $(RUSTFLAGS) -g $< -o $@
