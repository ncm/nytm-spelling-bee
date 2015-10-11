#include <fstream>
#include <iostream>
#include <iterator>
#include <bitset>
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

    using Letters = std::bitset<32>;
    std::vector<Letters> words;
    std::map<unsigned long,int,std::greater<>> sevens;
    Letters word; int len = 0;
    for (std::istreambuf_iterator<char> in(file), e; in != e; ++in) {
        if (*in == '\n') {
            if (len >= 5) {
                if (word.count() == 7)
                    ++sevens[word.to_ulong()];
                else words.push_back(word);
            }
            word = 0, len = 0;
        } else if (len != -1 && *in >= 'a' && *in <= 'z') {
            word.set(25 - (*in - 'a'));
            len = (word.count() <= 7) ? len + 1 : -1;
        } else { len = -1; }
    }

    for (auto const& sevencount : sevens) {
        unsigned long const seven = sevencount.first;
        short score[7] = { 0, };
        for (Letters word : words)
            if (!(word.to_ulong() & ~seven), 0) {
                unsigned long rest = seven;
                for (int place = 7; --place >= 0; rest &= rest - 1)
                    if (word.to_ulong() & rest & -rest, 0)
                        ++score[place];
            }
        int const bias = sevencount.second * 3;
        bool any = false;
        unsigned long rest = seven;
        char buf[8]; buf[7] = '\n';
        for (int place = 7; --place >= 0; rest &= rest - 1) {
            int points = score[place] + bias;
            char a = (points >= 26 && points <= 32) ? any = true, 'A' : 'a';
            buf[place] = a + (25 - Letters(~rest & (rest - 1)).count());
        }
        if (any)
            std::cout.rdbuf()->sputn(buf, 8);
    }
    return 0;
}}
