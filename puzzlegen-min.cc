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
    std::ios_base::sync_with_stdio(false);
    std::string const name = (ac == 1) ? "/usr/share/dict/words" : av[1];
    std::ifstream fs;
    std::istream& infile = (name != "-") ? (fs.open(name), fs) : std::cin;
    if (!infile)
        return std::cerr << "file open failed, " << name << '\n', 1;

    using Letters = unsigned;
    const Letters Z = 1;
    std::vector<Letters> words;
    std::set<Letters, std::greater<>> sevens;
    for (std::istream_iterator<std::string> in(infile), end; in != end; ++in) {
        if (in->size() >= 5) {
            Letters word = 0; int count = 0;
            auto wi = in->begin(), we = in->end();
            do  word |= (*wi >= 'a' && *wi <= 'z') ? Z << ('z' - *wi) : -Z;
            while ((count = __builtin_popcountl(word)) <= 7 && ++wi != we);
            if (count <= 7)
                words.push_back(word);
            if (count == 7)
                sevens.insert(word);
        }
    }

    for (Letters seven : sevens) {
        int score[7] = { 0, };
        for (Letters word : words)
            if (!(word & ~seven)) {
                const int points = (word == seven) ? 3 : 1;
                Letters rest = seven;
                for (int place = 7; --place >= 0; rest &= rest - 1)
                    if (word & rest & -rest)
                        score[place] += points;
            }
        bool any = false;
        Letters rest = seven;
        char buf[8] = { 0, 0, 0, 0, 0, 0, 0, '\n' };
        for (int place = 7; --place >= 0; rest &= rest - 1) {
            char z = 'z';
            if (score[place] >= 26 && score[place] <= 32)
                any = true, z = 'Z';
            buf[place] = z - __builtin_ctzl(rest);
        }
        if (any)
            std::cout.rdbuf()->sputn(buf, 8);
    }
    return 0;
}
