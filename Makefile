SHELL = bash  # to get "for ((...))" syntax
CXX = g++
RUSTC = rustc
STDLIB =
CXXFLAGS = -O3 $(STDLIB) -std=c++14 -Wall -march=corei7
RUSTFLAGS = -C opt-level=2 -C target-cpu=corei7
RUSTMKLIB = --crate-type=staticlib
LIBS = -lpthread -ldl
T=1

OFILES = puzzlegen-cc.o puzzlegen-rs.a
PROGRAMS = puzzlegen-cc puzzlegen-rs
BENCHES = cc.bench rs.bench all.bench

all: $(PROGRAMS) all.run article.html

# google benchmark binaries, https://github.com/google/benchmark

gbench.run: gbench
	./gbench --color_print=false --benchmark_min_time=$(T) | grep ........

gbench: gbench.cc puzzlegen-cc.o puzzlegen-rs.a
	$(CXX) -o gbench -std=c++14 $^ -L/usr/local/lib -lbenchmark $(LIBS)

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
	$(CXX) -g3 $(CXXFLAGS) $< -o $@

puzzlegen-rs: puzzlegen.rs
	$(RUSTC) -C debuginfo=2 $(RUSTFLAGS) $< -o $@

# independent bench binaries, run multiple times and report runtime.

all.bench: bench-all.cc $(OFILES)
	$(CXX) -o $@ -std=c++14 -DCC -DRS $^ $(LIBS)

cc.bench: bench-all.cc puzzlegen-cc.o
	$(CXX) -o $@ -DCC -std=c++14 -DCC $^

rs.bench: bench-all.cc puzzlegen-rs.a
	$(CXX) -o $@ -std=c++14 -DRS $^ $(LIBS)

# objects

puzzlegen-cc.o: puzzlegen.cc
	$(CXX) $(CXXFLAGS) -c -g0 -Dmain=cc_main $< -o $@

puzzlegen-rs.a: puzzlegen.rs
	$(RUSTC) $(RUSTFLAGS) $(RUSTMKLIB) -C debuginfo=0 --cfg main $< -o $@

#

.SUFFIXES:
.SUFFIXES: .bench .run
.bench.run:
	./$< | tee $<.out | wc -l
	for ((i=0;i<400;++i)); do cat out.ref; done | cmp $<.out -
	@echo OK

article.html: article.md
	pandoc -s --smart --template template.html article.md -o article.html
