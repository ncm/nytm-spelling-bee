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
        .filter_map(|c| Some(match (c as char, len) {
            ('\n', -1 ... 4) => { word = 0; len = 0; return None },
            (_, -1) => return None,
            ('\n', _) => { let out = word; word = 0; len = 0; out },
            ('a' ... 'z', _) => {
                    word |= A >> c - ('a' as u8); len += 1;
                    if word.count_ones() > 7
                        { len = -1 }
                    return None
                },
            (_, _) => { len = -1; return None }
        })).filter(|&word| { words.push(word); word.count_ones() == 7 })
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
        let (is_viable, _) = scores.iter().zip(out.iter_mut().rev().skip(1))
            .fold((false, seven), |(mut is_viable, rest), (&score, out)| {
                let a = match score + bias
                    { 26 ... 32 => { is_viable = true; 'A' }, _ => 'a' } as u8;
                *out = a + (25u32 - rest.trailing_zeros()) as u8;
                (is_viable, rest & rest - 1)
            });
         if is_viable
              { sink.write(&out).unwrap(); };
    }
}
