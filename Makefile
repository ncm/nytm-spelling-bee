SHELL = bash  # to get "for ((...))" syntax
CXX = g++
RUSTC = rustc
STDLIB =
OPTLEVEL=2
CXXFLAGS = -O$(OPTLEVEL) $(STDLIB) -g3 -std=c++14 -Wall -march=corei7
RUSTFLAGS = -C opt-level=$(OPTLEVEL) -C target-cpu=corei7 -g
RUSTMKLIB = --crate-type=staticlib

OFILES = puzzlegen-cc.o puzzlegen-rs.a
PROGRAMS = puzzlegen-cc puzzlegen-rs
BENCHES = cc.bench rs.bench all.bench

all: $(PROGRAMS) all.run

# "make all.run" benchmarks all versions.
# you can also (e.g.) "make rs.run" to run just one version

all.run: all.bench
	./$< | tee $<.out | wc -l
	for ((i=0;i<800;++i)); do cat out.ref; done | cmp - $<.out
	@echo OK

clean:; rm -f $(OFILES) $(PROGRAMS) $(BENCHES) *.bench.out

# 
# These are actual programs that actually generate, you know, puzzles.

puzzlegen-cc: puzzlegen.cc
	$(CXX) $(CXXFLAGS) $< -o $@

puzzlegen-rs: puzzlegen.rs
	$(RUSTC) $(RUSTFLAGS) $< -o $@

# bench binaries, run multiple times and report runtime.

all.bench: bench-all.cc $(OFILES)
	$(CXX) -o $@ -std=c++14 -DCC -DRS $^ -lpthread -ldl

cc.bench: bench-all.cc puzzlegen-cc.o
	$(CXX) -o $@ -DCC -std=c++14 -DCC $^

rs.bench: bench-all.cc puzzlegen-rs.a
	$(CXX) -o $@ -std=c++14 -DRS $^ -lpthread -ldl

# objects

puzzlegen-cc.o: puzzlegen.cc
	$(CXX) $(CXXFLAGS) -c -Dmain=cc_main $< -o $@

puzzlegen-rs.a: puzzlegen.rs
	$(RUSTC) $(RUSTFLAGS) $(RUSTMKLIB) --cfg main $< -o $@

#

.SUFFIXES:
.SUFFIXES: .bench .run
.bench.run: 
	./$< | tee $<.out | wc -l
	for ((i=0;i<400;++i)); do cat out.ref; done | cmp $<.out -
	@echo OK
