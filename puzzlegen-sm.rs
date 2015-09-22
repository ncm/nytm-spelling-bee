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

    let mut words : Vec<Letters> = Vec::new();
    let sevens : BTreeSet<_> = io::BufReader::new(file).bytes()
        .filter_map(|resultc| resultc.ok())
        .scan((0 as Letters, 0), |&mut (ref mut word, ref mut len), c|
            Some(match (c as char, *len) {
                ('\n', -1 ... 4) => { *word = 0; *len = 0; None },
                (_, -1) => None,
                ('\n', _) => { let w = *word; *word = 0; *len = 0;  Some(w) },
                ('a' ... 'z', _) => {
                    *word |= A >> c - ('a' as u8);
                    if word.count_ones() <= 7
                          { *len += 1; None }
                    else { *len = -1; None }
                },
                (_, _) => { *len = -1; None }
            })
        ).filter_map(|option| option)
        .filter(|&word| { words.push(word); word.count_ones() == 7 })
        .collect();

    let stdout = io::stdout();
    let mut sink = io::BufWriter::new(stdout.lock());
    for &seven in sevens.iter().rev() {
        let (scores, bias) = words.iter().map(|&word| word)
            .filter(|&word| word & !seven == 0)
            .fold(([0u16;7], 0u16), |(mut scores, mut bias), word| {
                if word == seven {
                   bias += 3;
                } else { scores.iter_mut().fold(seven, |rest, score| {
                   if word & rest & !(rest - 1) != 0
                       { *score += 1 }
                   rest & rest - 1
                });};
                (scores, bias)
            });
        let mut out = [0, 0, 0, 0, 0, 0, 0, '\n' as u8];
        let (any, _) = scores.iter().zip(out.iter_mut().rev().skip(1))
            .fold((false, seven), |(mut any, rest), (&score, out)| {
                let a = match score + bias
                    { 26 ... 32 => { any = true; 'A' }, _ => 'a' };
                *out = (a as u8) + (25u32 - rest.trailing_zeros()) as u8;
                (any, rest & rest - 1)
            });
         if any
              { sink.write(&out).unwrap(); };
    }
}
