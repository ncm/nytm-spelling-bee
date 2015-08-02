RTLIBPATH = 
CXX = g++
CXXFLAGS = -O3 -Wall -g3 -march=native -mtune=native

run: puzzlegen
	$(RTLIBPATH) time ./puzzlegen | tee out | wc -l
	cmp out.ref out

puzzlegen: puzzlegen.cc
	$(CXX) $(CXXFLAGS) -std=c++14 $< -o $@

