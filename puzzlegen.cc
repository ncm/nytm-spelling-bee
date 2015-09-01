#include <fstream>
#include <iostream>
#include <iterator>
#include <string>
#include <vector>
#include <set>
#include <functional>
#include <streambuf>
#include "bitset_set.h"

int main(int ac, char** av)
{
    std::ios_base::sync_with_stdio(false);
    std::string const name = (ac == 1) ? "/usr/share/dict/words" : av[1];
    std::ifstream fs;
    std::istream& in = (name != "-") ? (fs.open(name), fs) : std::cin;
    if (!in)
        return std::cerr << "file open failed, " << name << '\n', 1;

    using Letters = tst::bitset_set<'z'-'a'+1>;
    std::vector<Letters> words;
    std::set<Letters, std::greater<>> sevens;

    for (std::istream_iterator<std::string> it(in), end; it != end; ++it) {
        if (it->size() >= 5) {
            Letters word;
            for (char c : *it)
                (c >= 'a' && c <= 'z') ? word.set('z' - c) : word.set();
            if (word.count() <= 7)
                words.push_back(word);
            if (word.count() == 7)
                sevens.insert(word.to_ulong());
        }
    }

    char buf[8]; buf[7] = '\n';
    for (Letters const seven : sevens) {
        int score[7] = { 0, };
        for (Letters word : words)
            if ((word & ~seven).none()) {
                unsigned place = 7;
                for (Letters letter : seven) {
                    --place;
                    if ((word & letter).any())
                        score[place] += (word == seven) ? 3 : 1;
                }
            }
        bool any = false;
        unsigned place = 7;
        for (Letters letter : seven) {
            --place;
            bool middle = (score[place] > 25 && score[place] < 33);
            buf[place] = (middle ? 'Z' : 'z') - letter.least_bit_position();
            any |= middle;
        }
        if (any)
            std::cout.rdbuf()->sputn(buf, 8);
    }
    return 0;
}
