#include <fstream>
#include <iostream>
#include <iterator>
#include <string>
#include <vector>
#include <map>
#include <functional>
#include <streambuf>

extern "C" { int main(int ac, char** av)
{
    std::string const name = (ac == 1) ? "/usr/share/dict/words" : av[1];
    std::ifstream fs;
    std::istream& file = (name != "-") ? (fs.open(name), fs) : std::cin;
    if (!file)
        return std::cerr << "file open failed, " << name << '\n', 1;

    using Letters = unsigned;
    const Letters A = 1 << ('z' - 'a');
    std::vector<Letters> words;
    std::map<Letters,int,std::greater<>> sevens;
    for (std::istream_iterator<std::string> in(file), e; in != e; ++in) [&]{
        if (in->size() < 5)
            return;
        Letters word = 0;
        for (auto c : *in)
            if (c < 'a' || c > 'z' ||
                    (word |= A >> (c - 'a'), __builtin_popcountl(word) > 7))
                return;
        if (__builtin_popcountl(word) == 7)
            ++sevens[word];
        else words.push_back(word);
    }();

    for (auto const& sevencount : sevens) {
        const Letters seven = sevencount.first;
        short score[7] = { 0, };
        for (Letters word : words)
            if (!(word & ~seven)) {
                Letters rest = seven;
                for (int place = 7; --place >= 0; rest &= rest - 1)
                    if (word & rest & -rest)
                        ++score[place];
            }
        bool any = false; Letters rest = seven;
        short bias = sevencount.second * 3;
        char buf[8]; buf[7] = '\n';
        for (int place = 7; --place >= 0; rest &= rest - 1) {
            const int points = score[place] + bias;
            const char a = (points >= 26 && points <= 32) ?
                any = true, 'A' : 'a';
            buf[place] = a + (25 - __builtin_ctzl(rest));
        }
        if (any)
            std::cout.rdbuf()->sputn(buf, 8);
    }
    return 0;
}}
