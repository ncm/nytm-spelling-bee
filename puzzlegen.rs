use std::io::prelude::*;
use std::fs;
use std::env::args;
use std::collections::BTreeSet;

fn main() {
    let name = args().nth(1).unwrap_or(String::from("/usr/share/dict/words"));
    let file : Box<std::io::Read> = match &name as &str {
        "-" => Box::new(std::io::stdin()),
         _  => Box::new(fs::File::open(name).ok().expect("file open failed"))
    };

    type Letters = u32;
    const ZERO : Letters = 0; const ONE : Letters = 1;
    let mut words : Vec<Letters> = Vec::new();
    let sevens = std::io::BufReader::new(file).lines()
        .filter_map(|line| match line { Ok(s) => Some(s), _ => None })
        .filter(|line| line.len() >= 5)
        .map(|line| line.bytes().fold(ZERO,
             |word, c| match c as char {
                'a' ... 'z' => word | ONE << ('z' as u8) - c,
                         _  => !ZERO }))
        .filter(|word| word.count_ones() <= 7)
        .inspect(|word| words.push(*word))
        .filter(|word| word.count_ones() == 7)
        .collect::<BTreeSet<Letters>>();

    let mut sink = std::io::BufWriter::new(std::io::stdout());
    let mut out = [0u8;8]; out[7] = '\n' as u8;
    sevens.iter().rev().all(|seven| {
        let scores : [i32;7] = words.iter()
            .filter(|word| **word & !seven == 0)
            .map(|word| (word, if *word == *seven { 3 } else { 1 }))
            .fold([0;7],
                |mut scores, (word, points)| {
                    scores.iter_mut()
                        .fold(*seven, |rest, score| {
                            if word & rest & !(rest - 1) != 0 {
                                *score += points }
                            rest & rest - 1
                        });
                    scores
                });
        let (any, _, _) = scores.iter()
            .fold((false, *seven, 6), |(any, rest, i), score| {
                let (this, z) = match *score {
                    26 ... 32 => { (true,  'Z') },
                            _ => { (false, 'z') } };
                out[i] = (z as u8) - (rest.trailing_zeros() as u8);
                (any|this, rest & rest - 1, i - 1)
            });
        if any {
            sink.write(&out).unwrap(); }
        true
    });
} 
