use std::io::prelude::*;
use std::{fs,io,env};
use std::collections::BTreeSet;

const WORDS_FILE : &'static str = "/usr/share/dict/words";
type Letters = u32;
const Z : Letters = 1;

fn main() {
    let name = env::args().nth(1).unwrap_or(String::from(WORDS_FILE));
    let stdin = io::stdin();
    let file : Box<io::Read> = match &*name {
        "-" => Box::new(stdin.lock()),
        _   => Box::new(fs::File::open(name).ok().expect("file open failed"))
    };

    let mut words : Vec<Letters> = Vec::new();
    let mut word : Letters = 0;
    let mut len = 0;
    let sevens : BTreeSet<_> = io::BufReader::new(file).bytes()
        .filter_map(|c| c.ok())
        .filter_map(|c|
            match (c as char, len) {
                ('\n', -1 ... 4) => { word = 0; len = 0; None },
                ('\n', _) => { let out = Some(word); word = 0; len = 0; out },
                (_, -1) => None,
                ('a' ... 'z', _) => {
                    word |= Z << (('z' as u8) - c); len += 1; None },
                (_, _)   => { len = -1; None }
            })
        .filter(|&word| word.count_ones() <= 7)
        .inspect(|&word| words.push(word))
        .filter(|&word| word.count_ones() == 7)
        .collect();

    let stdout = io::stdout();
    let mut sink = io::BufWriter::new(stdout.lock());
    sevens.iter().rev().map(|&seven| {
        let scores = words.iter()
            .filter(|&&word| word & !seven == 0)
            .map(|&word| (word, if word == seven { 3 } else { 1 }))
            .fold([0;7], |mut scores, (word, points)| {
                scores.iter_mut().fold(seven, |rest, score| {
                    if word & rest & !(rest - 1) != 0 {
                        *score += points }
                    rest & rest - 1
                });
                scores
            });
        let mut out = [0, 0, 0, 0, 0, 0, 0, '\n' as u8];
        let (_, is_viable) = scores.iter().zip(out.iter_mut().rev().skip(1))
            .fold((seven, false), |(rest, is_viable), (&score, out)| {
                let (z, may_be_center) = match score {
                    26 ... 32 => ('Z' as u8, true),
                    _         => ('z' as u8, false)
                };
                *out = z - (rest.trailing_zeros() as u8);
                (rest & rest - 1, is_viable | may_be_center)
            });
         if is_viable {
              sink.write(&out).unwrap(); };
    }).count();
}
