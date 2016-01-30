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
    std::vector<unsigned> sevens; sevens.reserve(1<<14);
    std::bitset<32> word; int len = 0; bool skip = false;
    for (std::istreambuf_iterator<char> in(file), eof; in != eof; ++in) {
        if (*in == '\n') {
            if (!skip && len >= 5)
                (word.count() < 7 ? words : sevens).push_back(word.to_ulong());
            word = len = skip = false;
        } else if (!skip && *in >= 'a' && *in <= 'z') {
            word.set(25 - (*in - 'a'));
            if (word.count() <= 7) ++len; else skip = true;
        } else { skip = true; }
    }

    std::sort(sevens.begin(), sevens.end());
    std::vector<unsigned> counts; counts.resize(sevens.size());
    int count = -1; unsigned prev = 0;
    for (auto seven : sevens) {
        if (prev != seven)
            sevens[++count] = prev = seven;
        counts[count] += 3;
    }

    for (; count >= 0; --count) {
        unsigned const seven = sevens[count];
        int scores[7] = { 0, };
        for (unsigned word : words)
            if (!(word & ~seven)) {
                unsigned rest = seven;
                for (int place = 7; --place >= 0; rest &= rest - 1)
                    if (word & rest & -rest)
                        ++scores[place];
            }

        bool any = false; unsigned rest = seven; int threes = counts[count];
        char out[8]; out[7] = '\n';
        for (int place = 7; --place >= 0; rest &= rest - 1) {
            int points = scores[place] + threes;
            char a = (points >= 26 && points <= 32) ? any = true, 'A' : 'a';
            out[place] = a + (25 - std::bitset<32>(~rest & (rest - 1)).count());
        }
        if (any)
            std::cout.rdbuf()->sputn(out, 8);
    }
    return 0;
}
