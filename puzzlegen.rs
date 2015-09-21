use std::io::prelude::*;
use std::{env, io, fs};
use std::collections::BTreeSet;

const WORDS_FILE : &'static str = "/usr/share/dict/words";
type Letters = u32;
const A : Letters = 1 << 25;
const NONE : Letters = 0;
fn some_if<V>(p: bool, v: V) -> Option<V> { if p { Some(v) } else { None }}

fn main() {
    let name = env::args().nth(1).unwrap_or(String::from(WORDS_FILE));
    let stdin = io::stdin();
    let file : Box<io::Read> = match &*name {
        "-" => Box::new(stdin.lock()),
        _   => Box::new(fs::File::open(name).ok().expect("file open failed"))
    };

    let mut words : Vec<Letters> = Vec::new();
    let sevens : BTreeSet<_> = io::BufReader::new(file).lines()
        .filter_map(|line| line.ok())
        .filter(|line| line.len() >= 5)
        .filter_map(|line| line.bytes().scan(NONE, |word, c|
            some_if(word.count_ones() <= 7, match c as char {
                'a' ... 'z' => { *word |= A >> c - ('a' as u8); *word }
                _  => { *word = !NONE; *word }
            })).last())
        .filter(|&word| word.count_ones() <= 7)
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
                } else {
                    scores.iter_mut().fold(seven, |rest, score| {
                        if word & rest & !(rest - 1) != 0
                            { *score += 1 }
                        rest & rest - 1
                    });
                };
                (scores, bias)
            });
        let mut out = [0, 0, 0, 0, 0, 0, 0, '\n' as u8];
        let (is_viable, _) = scores.iter().zip(out.iter_mut().rev().skip(1))
            .fold((false, seven), |(mut is_viable, rest), (&score, out)| {
                let a = match score + bias
                    { 26 ... 32 => { is_viable = true; 'A' }, _ => 'a' } as u8;
                *out = a + (25 - (rest.trailing_zeros() as u8));
                (is_viable, rest & rest - 1)
            });
        if is_viable
            { sink.write(&out).unwrap(); };
    }
}
