use std::io::prelude::*;
use std::{env, io, fs};
use std::collections::BTreeMap;

const WORDS_FILE : &'static str = "/usr/share/dict/words";
type Letters = u32;
const A : Letters = 1 << 25;

#[no_mangle] pub extern fn str_rs_main() {
    let name = env::args().nth(1).unwrap_or(String::from(WORDS_FILE));
    let stdin = io::stdin();
    let file : Box<io::Read> = match &*name {
        "-" => Box::new(stdin.lock()),
        _   => Box::new(fs::File::open(name).ok().expect("file open failed"))
    };

    let mut sevens : BTreeMap<Letters,u16> = BTreeMap::new();
    let words : Vec<_> = io::BufReader::new(file).lines()
        .filter_map(|line| line.ok())
        .filter(|line| line.len() >= 5)
        .filter_map(|line| line.bytes()
            .scan(0 as Letters, |word, c|
                if word.count_ones() <= 7 {
                    *word |= match c as char {
                        'a' ... 'z' => A >> c - ('a' as u8),
                        _ => !(0 as Letters)
                    }; Some(*word)
                } else { None }).last())
        .filter(|&word| word.count_ones() <= 7)
        .filter_map(|word| if word.count_ones() < 7
                { Some(word) }
            else { *sevens.entry(word).or_insert(0) += 1; None })
        .collect();

    let stdout = io::stdout();
    let mut sink = io::BufWriter::new(stdout.lock());
    for (&seven, &count) in sevens.iter().rev() {
        let scores = words.iter().map(|&word| word)
            .filter(|&word| word & !seven == 0)
            .fold([0u16;7], |mut scores, word| {
                scores.iter_mut().fold(seven, |rest, score| {
                    if word & rest & !(rest - 1) != 0
                        { *score += 1 }
                    rest & rest - 1
                });
                scores
            });
        let mut out = [0, 0, 0, 0, 0, 0, 0, '\n' as u8];
        let bias = count * 3;
        let (any, _) = scores.iter().zip(out.iter_mut().rev().skip(1))
            .fold((false, seven), |(mut any, rest), (&score, out)| {
                let a = match score + bias
                    { 26 ... 32 => { any = true; 'A' }, _ => 'a' } as u8;
                *out = a + (25 - (rest.trailing_zeros() as u8));
                (any, rest & rest - 1)
            });
        if any
            { sink.write(&out).unwrap(); };
    }
}
#[cfg(not(main))] fn main() { str_rs_main(); }
