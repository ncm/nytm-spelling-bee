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
        int score[7] = { 0, };
        for (Letters word : words)
            if (!(word & ~seven)) {
                int points = (word == seven) ? 3 : 1;
                Letters rest = seven;
                for (int place = 7; --place >= 0; rest &= ~-rest) {
                    if (word & rest & -rest)
                        score[place] += points;
                }
            }
        bool any = false;
        Letters rest = seven;
        for (int place = 7; --place >= 0; rest &= ~-rest) {
            bool middle = (score[place] > 25 && score[place] < 33);
            any |= middle;
            buf[place] = (middle ? 'Z' : 'z') - __builtin_ctzl(rest & -rest);
        }

        if (any)
            std::cout.rdbuf()->sputn(buf, 8);
    }
    return 0;
}
