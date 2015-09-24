SHELL = bash  # to get "for ((...))" syntax
CXX = g++
RUSTC = rustc
STDLIB =
CXXFLAGS = -O2 $(STDLIB) -g3 -std=c++14 -Wall -march=corei7
RUSTFLAGS = -C opt-level=2 -C target-cpu=corei7 -g
RUSTMKLIB = --crate-type=staticlib

OFILES = puzzlegen-str-cc.o puzzlegen-cc.o puzzlegen-str-rs.a puzzlegen-rs.a
PROGRAMS = puzzlegen-str-cc puzzlegen-cc puzzlegen-str-rs puzzlegen-rs
BENCHES = str-cc.bench cc.bench str-rs.bench rs.bench all.bench

all: $(PROGRAMS) all.run

# "make all.run" benchmarks all versions.
# you can also (e.g.) "make rs.run" to run just one version

all.run: all.bench
	./$< | tee $<.out | wc -l
	for ((i=0;i<80;++i)); do cat out.ref; done | cmp - $<.out
	@echo OK

clean:; rm -f $(OFILES) $(PROGRAMS) $(BENCHES) *.bench.out

# 
# These are actual programs that actually generate, you know, puzzles.

puzzlegen-str-cc: puzzlegen-str.cc
	$(CXX) $(CXXFLAGS) $< -o $@

puzzlegen-cc: puzzlegen.cc
	$(CXX) $(CXXFLAGS) $< -o $@

puzzlegen-str-rs: puzzlegen-str.rs
	$(RUSTC) $(RUSTFLAGS) $< -o $@

puzzlegen-rs: puzzlegen.rs
	$(RUSTC) $(RUSTFLAGS) $< -o $@

# bench binaries, run multiple times and report runtime.

all.bench: bench-all.cc $(OFILES)
	$(CXX) -o $@ -std=c++14 -DSTRCC -DCC -DSTRRS -DRS $^ -lpthread -ldl

str-cc.bench: bench-all.cc puzzlegen-str-cc.o
	$(CXX) -o $@ -DCC -std=c++14 -DCC $^

cc.bench: bench-all.cc puzzlegen-cc.o
	$(CXX) -o $@ -DSMCC -std=c++14 -DSMCC $^

str-rs.bench: bench-all.cc puzzlegen-str-rs.a
	$(CXX) -o $@ -std=c++14 -DRS $^ -lpthread -ldl

rs.bench: bench-all.cc puzzlegen-rs.a
	$(CXX) -o $@ -std=c++14 -DSMRS $^ -lpthread -ldl

# objects

puzzlegen-str-cc.o: puzzlegen-str.cc
	$(CXX) $(CXXFLAGS) -c -Dmain=str_cc_main $< -o $@

puzzlegen-cc.o: puzzlegen.cc
	$(CXX) $(CXXFLAGS) -c -Dmain=cc_main $< -o $@

puzzlegen-str-rs.a: puzzlegen-str.rs
	$(RUSTC) $(RUSTFLAGS) $(RUSTMKLIB) --cfg main $< -o $@

puzzlegen-rs.a: puzzlegen.rs
	$(RUSTC) $(RUSTFLAGS) $(RUSTMKLIB) --cfg main $< -o $@

#

.SUFFIXES:
.SUFFIXES: .bench .run
.bench.run: 
	./$< | tee $<.out | wc -l
	for ((i=0;i<20;++i)); do cat out.ref; done | cmp $<.out -
	@echo OK
