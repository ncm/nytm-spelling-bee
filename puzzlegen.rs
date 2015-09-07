use std::io::prelude::*;
use std::{env, io, fs};
use std::collections::BTreeSet;

const WORDS_FILE : &'static str = "/usr/share/dict/words";
type Letters = u32;
const NONE : Letters = 0;
const Z : Letters = 1;

fn main() {
    let name = env::args().nth(1).unwrap_or(String::from(WORDS_FILE));
    let stdin = io::stdin();
    let file : Box<io::Read> = match &*name {
        "-" => Box::new(stdin.lock()),
        _   => Box::new(fs::File::open(name).ok().expect("file open failed"))
    };

    let mut words : Vec<Letters> = Vec::new();
    let sevens = io::BufReader::new(file).lines()
        .filter_map(|line| line.ok())
        .filter(|line| line.len() >= 5)
        .map(|line| line.bytes().fold(NONE, |word, c|
            match c as char {
                'a' ... 'z' => word | Z << ('z' as u8) - c,
                _  => !NONE
            }))
        .filter(|&word| word.count_ones() <= 7)
        .inspect(|&word| words.push(word))
        .filter(|&word| word.count_ones() == 7)
        .collect::<BTreeSet<Letters>>();

    let stdout = io::stdout();
    let mut sink = io::BufWriter::new(stdout.lock());
    sevens.iter().rev().map(|&seven| {
        let scores = words.iter()
            .filter(|&&word| word & !seven == 0)
            .map(|&word| (word, if word == seven { 3 } else { 1 }))
            .fold([0;7], |mut scores, (word, points)| {
                scores.iter_mut().fold(seven, |rest, score| {
                    if word & rest & !(rest - 1) != 0 {
                        *score += points
                    }
                    rest & rest - 1
                });
                scores
            });
        let mut out : [u8;8] = [0, 0, 0, 0, 0, 0, 0, '\n' as u8];
        let (_, _, is_viable) = scores.iter()
            .fold((6, seven, false), |(i, rest, is_viable), &score| {
                let (z, may_be_center) = match score {
                    26 ... 32 => ('Z' as u8, true),
                    _         => ('z' as u8, false)
                };
                out[i] = z - (rest.trailing_zeros() as u8);
                (i - 1, rest & rest - 1, is_viable | may_be_center)
            });
         if is_viable {
              sink.write(&out).unwrap();
         };
    }).count();
}
