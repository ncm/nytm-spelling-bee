#include <fstream>
#include <iostream>
#include <streambuf>
#include <iterator>
#include <string>
#include <vector>
#include <set>
#include <numeric>
#include <functional>
#include <bitset>

using Letters = std::bitset<'z' - 'a' + 1>;

int main(int ac, char** av)
{
    std::string const name = (ac == 1) ? "/usr/share/dict/words" : av[1];
    std::ifstream fs;
    std::istream& in = (name != "-") ? (fs.open(name), fs) : std::cin;
    if (!in)
        return std::cerr << "file open failed, " << name << '\n', 1;

    std::vector<Letters> words;
    std::set<unsigned long, std::greater<>> sevens;

    for (std::istream_iterator<std::string> it(in), end; it != end; ++it) {
        if (it->size() >= 5) {
            Letters word = std::accumulate(it->begin(), it->end(), Letters{},
                [](Letters a, char b) {
                    return (b < 'a' || b > 'z') ? a.set() : a.set('z' - b);
                });
            if (word.count() <= 7)
                words.push_back(word);
            if (word.count() == 7)
                sevens.insert(word.to_ulong());
        }
    }

    char buf[8]; buf[7] = '\n';
    for (Letters seven : sevens) {
        auto for_each_in_seven = [seven](auto bin_op) {
            auto least_bit = [](Letters letters) {
                    return letters & Letters{-letters.to_ulong()}; };
            int place = 0;
            Letters rest = seven;
            do  bin_op(least_bit(rest), 6 - place);
            while (++place, rest &= ~least_bit(rest), rest.any());
        };
        int score[7] = { 0, };
        for (Letters word : words)
            if ((word & ~seven).none())
                for_each_in_seven([&](Letters letter, int place) {
                    if ((word & letter).any())
                        score[place] += (word == seven) ? 3 : 1;
                });
        bool any = false, mid;
        for_each_in_seven([&](Letters letter, int place) {
            auto position_of_bit = [](Letters letter) {
                    return Letters{letter.to_ulong() - 1}.count(); };
            any |= mid = (score[place] > 20 && score[place] < 33);
            buf[place] = (mid ? 'Z' : 'z') - position_of_bit(letter);
        });
        if (any)
            std::cout.rdbuf()->sputn(buf, 8);
    }
}
