use std::io::prelude::*;
use std::{fs,io,env};
use std::collections::BTreeSet;

const WORDS_FILE : &'static str = "/usr/share/dict/words";
type Letters = u32;
const A : Letters = 1 << 25;

fn main() {
    let name = env::args().nth(1).unwrap_or(String::from(WORDS_FILE));
    let stdin = io::stdin();
    let file : Box<io::Read> = match &*name {
        "-" => Box::new(stdin.lock()),
        _   => Box::new(fs::File::open(name).ok().expect("file open failed"))
    };

    let (mut word, mut len) = (0 as Letters, 0);
    let mut words : Vec<Letters> = Vec::new();
    let sevens : BTreeSet<_> = io::BufReader::new(file).bytes()
        .filter_map(|resultc| resultc.ok())
        .filter_map(|c| match (c as char, len) {
            ('\n', -1 ... 4) => { word = 0; len = 0; None },
            (_, -1) => None,
            ('\n', _) => {
                    let out = Some(word);
                    words.push(word); word = 0; len = 0; return out
                },
            ('a' ... 'z', _) => {
                word |= A >> c - ('a' as u8); len += 1;
                if word.count_ones() > 7 {
                    len = -1 }
                None
            },
            (_, _) => { len = -1; None }
        }).filter(|&word| word.count_ones() == 7)
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
        let (is_viable, _) = scores.iter().zip(out.iter_mut().rev().skip(1))
            .fold((false, seven), |(mut is_viable, rest), (&score, out)| {
                let a = match score {
                    26 ... 32 => { is_viable = true; 'A' as u8 },
                    _         => 'a' as u8
                };
                *out = a + (25 - rest.trailing_zeros()) as u8;
                (is_viable, rest & rest - 1)
            });
         if is_viable {
              sink.write(&out).unwrap(); };
    }).count();
}
