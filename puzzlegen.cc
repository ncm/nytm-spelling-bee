#include <fstream>
#include <iterator>
#include <string>
#include <vector>
#include <set>
#include <algorithm>
#include <iostream>

using Letters = int;

int main(int ac, char** av)
{
    std::string const name = (ac == 1) ? "/usr/share/dict/words" : av[1];
    std::ifstream fs;
    std::istream& in = (name != "-") ? (fs.open(name), fs) : std::cin;
    if (!in) {
       std::cerr << "file open failed, " << name << '\n';
       return 1;
    }

    std::vector<Letters> words;
    std::set<Letters, std::greater<Letters>> sevens;
    std::istream_iterator<std::string> it(in), end;
    std::for_each(it, end,
        [&](auto&& word) {
            Letters letters = std::accumulate(word.begin(), word.end(), 0,
                [](Letters a, char b) {
                    return a | ((b >= 'a' && b <= 'z') ? 1 << ('z' - b) : -1);
                }
            );
            if (letters > 0 && word.size() >= 5) {
                words.emplace_back(letters);
                if (__builtin_popcountl(letters) == 7)
                    sevens.insert(letters);
            }
        });

    for (Letters seven : sevens) {
        auto for_letters = [&seven](auto op) {
            int pos = 6;
            for (Letters rest = seven; rest != 0; --pos, rest &= ~-rest)
                op(rest & -rest, pos);
        };
        int points[7] = { 0, };
        for (Letters word : words)
            if ((word & ~seven) == 0)
                for_letters([&](Letters letter, int pos) {
                        if (word & letter)
                            points[pos] += (word == seven) ? 3 : 1;
                    });
        char buf[7];
        bool any = false, mid;
        for_letters([&](Letters letter, int pos) {
                any |= mid = (points[pos] > 20 && points[pos] < 33);
                buf[pos] = (mid? 'Z' : 'z') - __builtin_popcountl(letter - 1);
            });
        if (any)
            std::cout << buf << '\n';
    }
}
