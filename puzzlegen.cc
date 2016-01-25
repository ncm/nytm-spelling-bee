#include <iostream>
#include <fstream>
#include <iterator>
#include <bitset>
#include <vector>
#include <string>
#include <algorithm>
#include <functional>

extern "C" int main(int argc, char** argv)
{
    std::string name = (argc > 1) ? argv[1] : "/usr/share/dict/words";
    std::ifstream fs;
    std::istream& file = name == "-" ? std::cin : (fs.open(name), fs);
    if (!file)
        return std::cerr << "file open failed, \"" << name << "\"\n", 1;

    std::vector<unsigned> words; words.reserve(1<<15);
    std::vector<std::pair<unsigned,short>> sevens; sevens.reserve(1<<15);
    std::bitset<32> word; int len = 0;
    for (std::istreambuf_iterator<char> in(file), eof; in != eof; ++in) {
        if (*in == '\n') {
            if (len >= 5) {
                if (word.count() == 7) {
                    sevens.emplace_back(word.to_ulong(), 0);
                } else words.push_back(word.to_ulong());
            }
            word = 0, len = 0;
        } else if (len != -1 && *in >= 'a' && *in <= 'z') {
            word.set(25 - (*in - 'a'));
            len = (word.count() <= 7) ? len + 1 : -1;
        } else { len = -1; }
    }
    std::sort(sevens.begin(), sevens.end(), std::greater<>());
    auto p = sevens.begin();
    for (auto s = p; s != sevens.end(); ++p->second, ++s)
        if (s->first != p->first)
            *++p = *s;
    sevens.resize(p + 1 - sevens.begin());

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
