RTLIBPATH = 
CXX = g++
CXXFLAGS = -O3 -Wall -g3

run: puzzlegen
	$(RTLIBPATH) time ./puzzlegen | tee out | wc -l

puzzlegen: puzzlegen.cc
	$(CXX) $(CXXFLAGS) -std=c++14 $< -o $@

