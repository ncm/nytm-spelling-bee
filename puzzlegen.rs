use std::io::prelude::*;
use std::io::{self, BufReader};
use std::fs;
use std::env;
use std::collections::BTreeSet;

type Letters = u32;
const ZERO: Letters = 0;
const ONE: Letters = 1;

fn main() {
    let name = env::args().nth(1).unwrap_or(String::from("/usr/share/dict/words"));

    let stdin = io::stdin();

    let file : Box<io::Read> = match &*name {
        "-" => Box::new(stdin.lock()),
        _  => Box::new(fs::File::open(name).ok().expect("file open failed"))
    };

    let mut words : Vec<Letters> = Vec::new();

    let sevens = BufReader::new(file).lines()
        .filter_map(|line| match line { Ok(s) => Some(s), _ => None })
        .filter(|line| line.len() >= 5)
        .map(|line| line.bytes().fold(ZERO, |word, c|
            match c as char {
                'a' ... 'z' => word | ONE << ('z' as u8) - c,
                _  => !ZERO
            })
        )
        .filter(|word| word.count_ones() <= 7)
        .inspect(|&word| words.push(word))
        .filter(|word| word.count_ones() == 7)
        .collect::<BTreeSet<Letters>>();

    let stdout = io::stdout();
    let mut sink = stdout.lock();

    let mut out = [0u8;8];
    out[7] = '\n' as u8;

    sevens.iter().rev().all(|seven| {
        let scores = words.iter()
            .filter(|&&word| word & !seven == 0)
            .map(|word| (word, if word == seven { 3 } else { 1 }))
            .fold([0;7], |mut scores, (word, points)| {
                scores.iter_mut().fold(*seven, |rest, score| {
                    if word & rest & !(rest - 1) != 0 {
                        *score += points
                    }
                    rest & rest - 1
                });

              scores
            });

        let (any_centers, _, _) = scores.iter()
            .fold((false, *seven, 6), |(any_centers, rest, i), score| {
                let (this, z) = match *score {
                    26 ... 32 => { (true,  'Z') },
                    _ => { (false, 'z') }
                };

                out[i] = (z as u8) - (rest.trailing_zeros() as u8);

                (any_centers|this, rest & rest - 1, i - 1)
            });

        if any_centers {
            sink.write(&out).unwrap();
        }

        true
    });
} 
