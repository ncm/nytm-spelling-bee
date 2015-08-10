RTLIBPATH = 
CXX = g++
STDLIB =
CXXFLAGS = -O3 $(STDLIB) -Wall -g3 -march=native -mtune=native

run: puzzlegen
	$(RTLIBPATH) time ./puzzlegen | tee out | wc -l
	cmp out.ref out

puzzlegen: puzzlegen.cc bitset_set.h
	$(CXX) $(CXXFLAGS) -std=c++14 $< -o $@

