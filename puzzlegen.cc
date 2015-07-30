#include <fstream>
#include <iterator>
#include <string>
#include <vector>
#include <set>
#include <algorithm>
#include <iostream>

using Letters = int;
struct Word {
    Letters letters;
    size_t length;
};

int main(int ac, char** av)
{
    std::string const name = (ac == 1) ? "/usr/share/dict/words" : av[1];
    std::ifstream fs;
    std::istream& in = (name != "-") ? (fs.open(name), fs) : std::cin;
    if (!in) {
       std::cerr << "file open failed, " << name << '\n';
       return 1;
    }

    std::vector<Word> words;
    std::set<Letters> sevens;
    std::istream_iterator<std::string> it(in), end;
    std::for_each(it, end,
        [&words,&sevens](auto&& word) {
            Letters letters = std::accumulate(word.begin(), word.end(), 0,
                [](Letters a, char b) {
                    return a | ((b >= 'a' && b <= 'z') ? (1 << (b - 'a')) : -1);
                }
            );
            if (letters > 0 && word.size() >= 5) {
                words.emplace_back(Word{letters, word.size()});
                if (__builtin_popcountl(letters) == 7)
                    sevens.insert(letters);
            }
        });

    for (Letters seven : sevens) {
        auto for_letters = [&seven](auto op) {
            int pos = 0;
            for (Letters rest = seven; rest != 0; ++pos, rest &= ~-rest)
                op(rest & -rest, pos);
        };
        int points[7] = { 0, };
        for (Word& word : words)
            if ((word.letters & ~seven) == 0)
                for_letters([&seven, &word, &points](Letters letter, int pos) {
                        if (word.letters & letter)
                            points[pos] += (word.letters == seven) ? 3 : 1;
                    });
        char buf[7];
        bool any = false, mid;
        for_letters([&points, &buf, &any, &mid](Letters letter, int pos) { 
                any |= mid = (points[pos] > 25 && points[pos] < 33); 
                buf[pos] = (mid? 'A' : 'a') + __builtin_popcountl(letter - 1);
            });
        if (any)
            std::cout << buf << '\n';
    }
} 
