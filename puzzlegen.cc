#include <fstream>
#include <iostream>
#include <bitset>
#include <iterator>
#include <string>
#include <vector>
#include <set>
#include <functional>
#include <streambuf>

int main(int ac, char** av)
{
    std::string const name = (ac == 1) ? "/usr/share/dict/words" : av[1];
    std::ifstream fs;
    std::istream& in = (name != "-") ? (fs.open(name), fs) : std::cin;
    if (!in)
        return std::cerr << "file open failed, " << name << '\n', 1;

    using Letters = std::bitset<'z'-'a'+1>;
    std::vector<Letters> words;
    std::set<unsigned long, std::greater<>> sevens;

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
    for (Letters seven : sevens) {
        auto for_each_in_seven = [seven](auto bin_op) {
            auto least_bit = [](Letters word) -> Letters {
                return word & Letters{-word.to_ulong()}; };
            int place = 0;
            Letters rest = seven;
            do { bin_op(least_bit(rest), 6 - place);
            } while (++place, (rest &= ~least_bit(rest)).any());
        };
        int score[7] = { 0, };
        for (Letters word : words)
            if ((word & ~seven).none())
                for_each_in_seven([&](Letters letter, int place) {
                    if ((word & letter).any())
                        score[place] += (word == seven) ? 3 : 1;
                });
        bool any = false;
        for_each_in_seven([&](Letters letter, int place) {
            auto bit_position = [](Letters letter) {
                return Letters{letter.to_ulong() - 1}.count(); };
            bool middle = (score[place] > 20 && score[place] < 33);
            buf[place] = (middle ? 'Z' : 'z') - bit_position(letter);
            any |= middle;
        });
        if (any)
            std::cout.rdbuf()->sputn(buf, 8);
    }
}
