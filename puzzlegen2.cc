#include <fstream>
#include <iterator>
#include <vector>
#include <set>
#include <algorithm>
#include <iostream>
#include <streambuf>
#include <fstream>

using Letters = int;

int main(int ac, char** av)
{
    char const* name = (ac == 1) ? "/usr/share/dict/words" : av[1];
    std::filebuf fs;
    if (!(ac > 1 && av[1][0] == '-' && av[1][1] == '\0'))
        if (!fs.open(name, std::ios_base::in))
            return std::cerr << "file open failed, " << name << '\n', 1;

    std::istreambuf_iterator<char> end,
        in = fs.is_open() ? &fs : std::cin.rdbuf();
    std::vector<Letters> words;
    std::set<Letters, std::greater<Letters>> sevens;

    Letters word = 0;
    int length = 0, count = 0;
    std::for_each(in, end, [&](int c) {
        if (c == '\n') {
            if (length >= 5) {
                if (count <= 7) {
                    words.push_back(word);
                    if (count == 7)
                        sevens.insert(word);
                }
            }
            word = length = count = 0;
        } else if (length < 0) {
        } else if (c >= 'a' && c <= 'z') {
            ++length;
            Letters letter = (1 << ('z' - c));
            if (!(word & letter)) {
                word |= letter;
                if (++count > 7)
                    length = -1;
            }
        } else 
            length = -1;
    });

    char buf[] = "aaaaaaa\n";
    for (Letters seven : sevens) {
        auto for_each_letter = [&seven](auto op) {
            int pos = 0;
            for (Letters rest = seven; rest != 0; ++pos, rest &= ~-rest)
                op(rest & -rest, pos);
        };
        int points[7] = { 0, };
        for (Letters word : words)
            if ((word & ~seven) == 0)
                for_each_letter([&](Letters letter, int rvpos) {
                    if (word & letter)
                        points[6 - rvpos] += (word == seven) ? 3 : 1;
                });
        bool any = false, mid;
        for_each_letter([&](Letters letter, int rvpos) {
            int pos = 6 - rvpos;
            any |= mid = (points[pos] > 20 && points[pos] < 33);
            buf[pos] = (mid? 'Z' : 'z') - __builtin_popcountl(letter - 1);
        });
        if (any)
            std::cout.rdbuf()->sputn(buf, 8);
    }
}
