SHELL = bash
CXX = g++
RUSTC = rustc
STDLIB =
CXXFLAGS = -O3 $(STDLIB) -g3 -std=c++14 -Wall -march=corei7
RUSTFLAGS = -C opt-level=2 -C target-cpu=corei7 -g
RUSTMKLIB = --crate-type=staticlib

OFILES = puzzlegen-cc.o puzzlegen-sm-cc.o puzzlegen-rs.a puzzlegen-sm-rs.a

all.run: all.bench
	./$< | tee $<.out | wc -l
	for ((i=0;i<80;++i)); do cat out.ref; done | cmp - $<.out

# run-any: bench-any
# 	./bench-any | wc -l

all.bench: bench-all.cc $(OFILES)
	$(CXX) -o $@ -std=c++14 -DCC -DSMCC -DRS -DSMRS $^ -lpthread -ldl

# bench-any: bench-all.cc $(OFILES)
# 	CXX -o $@ -std=c++14 -DCC -DSMCC -DRS -DSMRS $?

cc.bench: bench-all.cc puzzlegen-cc.o
	$(CXX) -o $@ -DCC -std=c++14 -DCC $^

sm-cc.bench: bench-all.cc puzzlegen-sm-cc.o
	$(CXX) -o $@ -DSMCC -std=c++14 -DSMCC $^

rs.bench: bench-all.cc puzzlegen-rs.a
	$(CXX) -o $@ -std=c++14 -DRS $^ -lpthread -ldl

sm-rs.bench: bench-all.cc puzzlegen-sm-rs.a
	$(CXX) -o $@ -std=c++14 -DSMRS $^ -lpthread -ldl

#

puzzlegen-cc.o: puzzlegen.cc
	$(CXX) $(CXXFLAGS) -c -Dmain=cc_main $< -o $@

puzzlegen-sm-cc.o: puzzlegen-sm.cc
	$(CXX) $(CXXFLAGS) -c -Dmain=sm_cc_main $< -o $@

puzzlegen-rs.a: puzzlegen.rs
	$(RUSTC) $(RUSTFLAGS) $(RUSTMKLIB) --cfg main $< -o $@

puzzlegen-sm-rs.a: puzzlegen-sm.rs
	$(RUSTC) $(RUSTFLAGS) $(RUSTMKLIB) --cfg main $< -o $@

# 

puzzlegen-cc: puzzlegen.cc
	$(CXX) $(CXXFLAGS) $< -o $@

puzzlegen-sm-cc: puzzlegen-sm.cc
	$(CXX) $(CXXFLAGS) $< -o $@

puzzlegen-rs: puzzlegen.rs
	$(RUSTC) $(RUSTFLAGS) $< -o $@

puzzlegen-sm-rs: puzzlegen-sm.rs
	$(RUSTC) $(RUSTFLAGS) $< -o $@

#

.SUFFIXES:
.SUFFIXES: .bench .run .cc .rs
.bench.run: 
	./$< | tee $<.out | wc -l
	for ((i=0;i<20;++i)); do cat out.ref; done | cmp $<.out -
