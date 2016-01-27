#include <iostream>
#include <fstream>
#include <iterator>
#include <bitset>
#include <vector>
#include <string>
#include <algorithm>

extern "C" int main(int argc, char** argv)
{
    std::string name = (argc > 1) ? argv[1] : "/usr/share/dict/words";
    std::ifstream fs;
    std::istream& file = name == "-" ? std::cin : (fs.open(name), fs);
    if (!file)
        return std::cerr << "file open failed, \"" << name << "\"\n", 1;

    std::vector<unsigned> words; words.reserve(1<<15);
    std::vector<std::pair<unsigned,short>> sevens; sevens.reserve(1<<15);
    std::bitset<32> word; int len = 0; bool skip = false;
    for (std::istreambuf_iterator<char> in(file), eof; in != eof; ++in) {
        if (*in == '\n') {
            if (!skip && len >= 5) {
                if (word.count() == 7) {
                    sevens.emplace_back(word.to_ulong(), 0);
                } else words.push_back(word.to_ulong());
            }
            word = len = skip = false;
        } else if (!skip && *in >= 'a' && *in <= 'z') {
            word.set(25 - (*in - 'a'));
            if (word.count() <= 7) ++len; else skip = true;
        } else { skip = true; }
    }

    std::sort(sevens.begin(), sevens.end(),
        [](auto a, auto b) { return a.first > b.first; });
    size_t place = 0;
    for (auto seven : sevens) {
        if (sevens[place].first != seven.first)
            sevens[++place] = seven;
        ++sevens[place].second;
    }
    if (!sevens.empty()) sevens.resize(place + 1);

    for (auto sevencount : sevens) {
        unsigned const seven = sevencount.first;
        short scores[7] = { 0, };
        for (unsigned word : words)
            if (!(word & ~seven)) {
                unsigned rest = seven;
                for (int place = 7; --place >= 0; rest &= rest - 1)
                    if (word & rest & -rest)
                        ++scores[place];
            }

        bool any = false;
        unsigned rest = seven;
        char buf[8]; buf[7] = '\n';
        for (int place = 7; --place >= 0; rest &= rest - 1) {
            int points = scores[place] + sevencount.second * 3;
            char a = (points >= 26 && points <= 32) ? any = true, 'A' : 'a';
            buf[place] = a + (25 - std::bitset<32>(~rest & (rest - 1)).count());
        }
        if (any)
            std::cout.rdbuf()->sputn(buf, 8);
    }
    return 0;
}
