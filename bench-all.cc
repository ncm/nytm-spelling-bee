#include <iostream>
#include <sys/time.h>

extern "C" int cc_main(int ac, char** av);
extern "C" int sm_cc_main(int ac, char** av);
extern "C" int rs_main();
extern "C" int sm_rs_main();

auto run = [](char const* n, auto f) {
    timeval before, after;
    gettimeofday(&before, nullptr);
    for (int t = 0; t < 20; ++t)
        f();
    gettimeofday(&after, nullptr);
    std::cerr << ((after.tv_sec - before.tv_sec) * 1000000 +
                  (after.tv_usec - before.tv_usec)) / 1000000.0
              << ' ' << n << '\n';
};

int main(int ac, char** av)
{
#ifdef CC
    run("cc", [ac,av]() { cc_main(ac, av); });
#endif
#ifdef RS
    run("rs", rs_main);
#endif
#ifdef SMCC
    run("sm-cc", [ac,av]() { sm_cc_main(ac, av); });
#endif
#ifdef SMRS
    run("sm-rs", sm_rs_main);
#endif
}
