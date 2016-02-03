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
    std::bitset<32> word; int len = 0; int ones = 0;
    for (std::istreambuf_iterator<char> in(file), eof; in != eof; ++in) {
        if (*in == '\n') {
            if (len >= 5 && ones <= 7)
                (ones == 7 ? sevens : words).push_back(word.to_ulong());
            word = len = ones = 0;
        } else if (ones != 8 && *in >= 'a' && *in <= 'z') {
            ++len, ones = word.set(25 - (*in - 'a')).count();
        } else { ones = 8; }
    }

    std::sort(sevens.begin(), sevens.end());
    std::vector<int> counts; counts.resize(sevens.size());
    int count = -1; unsigned prev = 0;
    for (auto seven : sevens) {
        if (prev != seven)
            sevens[++count] = prev = seven;
        counts[count] += 3;
    }

    for (; count >= 0; --count) {
        unsigned const seven = sevens[count];
        int bits[7], scores[7];
        for (unsigned rest = seven, place = 7; place-- != 0; rest &= rest - 1) {
            bits[place] = std::bitset<32>((rest & ~(rest - 1)) - 1).count();
            scores[place] = counts[count];
        }
        for (unsigned word : words)
            if (!(word & ~seven))
                for (int place = 0; place < 7; ++place)
                    scores[place] += (word >> bits[place]) & 1;

        bool any = false; char out[8];
        for (int place = 0; place != 7; ++place) {
            int points = scores[place];
            char a = (points >= 26 && points <= 32) ? any = true, 'A' : 'a';
            out[place] = a + (25 - bits[place]);
        }
        if (any)
             out[7] = '\n', std::cout.rdbuf()->sputn(out, 8);
    }
    return 0;
}
