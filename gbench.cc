#include <benchmark/benchmark_api.h>

extern "C" int cc_main(int ac, char** av);
extern "C" int rs_main();

void cc(benchmark::State& state) {
    while (state.KeepRunning()) {
        static char arg[] = "";
        static char* args[] = { arg, 0 };
        cc_main(1, args);
    }
}

void rs(benchmark::State& state) {
    while (state.KeepRunning()) {
        rs_main();
    }
}

BENCHMARK(cc);
BENCHMARK(rs);
BENCHMARK_MAIN()
