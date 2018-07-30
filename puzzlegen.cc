#include <iostream>
#include <fstream>
#include <bitset>
#include <vector>
#include <algorithm>

extern "C" int main(int argc, char** argv)
{
    char const* fn = (argc > 1) ? argv[1] : "/usr/share/dict/words";
    std::ifstream fs;
    std::istream& file = (*fn != '-' || fn[1]) ? fs.open(fn), fs : std::cin;
    if (!file)
        return std::cerr << "file open failed, \"" << fn << "\"\n", 1;

    struct pangram { int word; short score; };
    std::vector<pangram> pangrams; pangrams.reserve(1<<14);
    std::vector<int> words; words.reserve(1<<15);
    int word = 0, len = 0, ones = 0;
    for (std::istreambuf_iterator<char> in(file), eof; in != eof; ++in) {
        if (*in == '\n' || *in == '\r') {
            if (len >= 5 && ones <= 7) {
                if (ones == 7) pangrams.push_back({word, 0});
                else words.push_back(word);
            }
            word = len = ones = 0;
        } else if (ones != 8 && *in >= 'a' && *in <= 'z') {
            int bit = (1<<25) >> (*in - 'a');
            ++len, ones += ((~word & bit) != 0), word |= bit;
        } else { ones = 8; }
    }

    std::sort(pangrams.begin(), pangrams.end(),
        [](auto a, auto b) { return a.word > b.word; });
    int back = 0, prev = pangrams.front().word;
    for (auto gram : pangrams) {
        if (prev != gram.word)
            pangrams[++back].word = prev = gram.word;
        pangrams[back].score += 3;
    }
    pangrams.resize(back + 1);

    for (auto gram : pangrams) {
        char out[8] = { 0, };
        for (int pos = 7, rest = gram.word; pos-- != 0; rest &= rest - 1)
            out[pos] = 25 - std::bitset<32>((rest & ~(rest - 1)) - 1).count();
        short scores[7] = { 0, };
        for (int word : words)
            if (!(word & ~gram.word))
                for (int pos = 0; pos < 7; ++pos)
                    scores[pos] += (word & ((1<<25) >> out[pos])) != 0;

        for (int pos = 0; pos < 7; ++pos) {
            int sc = scores[pos] + gram.score;
            out[pos] += (sc >= 26 && sc <= 32) ? out[7] = '\n', 'A' : 'a';
        }
        if (out[7])
            std::cout.rdbuf()->sputn(out, 8);
    }
    return 0;
}
