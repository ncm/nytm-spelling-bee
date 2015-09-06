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
    for &seven in sevens.iter().rev() {
        let mut scores = [0i32;7];
        // for &word in words.iter() {
        for &word in &words {
            if word & !seven == 0 {
                let points = if word == seven { 3 } else { 1 };
                let mut rest = seven;
                for score in &mut scores {
                    if word & rest & !(rest - 1) != 0 {
                        *score += points
                    }
                    rest &= rest - 1
                }
            }
        }
        let (mut rest, mut i, mut is_viable, mut out) = (
                 seven, 0, false, [0,0,0,0,0,0,0,'\n' as u8]);
        while rest != 0 {
            let z = match scores[i] {
                26 ... 32 => { is_viable = true; 'Z' as u8 },
                _         => 'z' as u8
            };
            out[6 - i] = z - (rest.trailing_zeros() as u8);
            rest &= rest - 1; i += 1;
        }
        if is_viable {
             sink.write(&out).unwrap();
        };
    }
} 
