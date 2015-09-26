#include <iostream>
#include <sys/time.h>

extern "C" int str_cc_main(int ac, char** av);
extern "C" int cc_main(int ac, char** av);
extern "C" int str_rs_main();
extern "C" int rs_main();

auto run = [](char const* name, auto f) {
    timeval before, after;
    gettimeofday(&before, nullptr);
    for (int t = 0; t < 200; ++t)
        f();
    gettimeofday(&after, nullptr);

    long long t = (after.tv_sec - before.tv_sec) * 1000000 +
                      (after.tv_usec - before.tv_usec);
    std::cerr << t / 1000000.0 << ' ' << name << '\n';
};

int main(int ac, char** av)
{
#ifdef STRCC
    run("str-cc", [ac,av]() { str_cc_main(ac, av); });
#endif
#ifdef CC
    run("cc", [ac,av]() { cc_main(ac, av); });
#endif
#ifdef STRRS
    run("str-rs", str_rs_main);
#endif
#ifdef RS
    run("rs", rs_main);
#endif
#ifdef RS
    run("rs", rs_main);
#endif
#ifdef STRRS
    run("str-rs", str_rs_main);
#endif
#ifdef CC
    run("cc", [ac,av]() { cc_main(ac, av); });
#endif
#ifdef STRCC
    run("str-cc", [ac,av]() { str_cc_main(ac, av); });
#endif
}
