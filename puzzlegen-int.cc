#include <fstream>
#include <iostream>
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

    using Letters = unsigned;
    std::vector<Letters> words;
    std::set<Letters, std::greater<>> sevens;

    for (std::istream_iterator<std::string> it(in), end; it != end; ++it) {
        if (it->size() >= 5) {
            Letters word = 0;
            for (char c : *it)
                word |= (c >= 'a' && c <= 'z') ? (1u << ('z' - c)) : -1u;
            int count = __builtin_popcountl(word);
            if (count <= 7)
                words.push_back(word);
            if (count == 7)
                sevens.insert(word);
        }
    }

    char buf[8]; buf[7] = '\n';
    for (Letters seven : sevens) {
        auto for_each_in_seven = [seven](auto bin_op) {
            auto least_bit = [](Letters word) -> Letters {
                return word & -word; };
            int place = 0;
            Letters rest = seven;
            do { bin_op(least_bit(rest), 6 - place);
            } while (++place, (rest &= ~least_bit(rest)));
        };
        int score[7] = { 0, };
        for (Letters word : words)
            if (!(word & ~seven))
                for_each_in_seven([&](Letters letter, int place) {
                    if (word & letter)
                        score[place] += (word == seven) ? 3 : 1;
                });
        bool any = false;
        for_each_in_seven([&](Letters letter, int place) {
            auto bit_position = [](Letters letter) {
                return __builtin_popcountl(letter - 1); };
            bool middle = (score[place] > 25 && score[place] < 33);
            buf[place] = (middle ? 'Z' : 'z') - bit_position(letter);
            any |= middle;
        });
        if (any)
            std::cout.rdbuf()->sputn(buf, 8);
    }
    return 0;
}
