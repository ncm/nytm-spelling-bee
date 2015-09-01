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
    let mut words : Vec<Letters> = Vec::new();
    let sevens = std::io::BufReader::new(file).lines()
        .filter_map(|line| match line { Ok(s) => Some(s), _ => None })
        .filter(|line| line.len() >= 5)
        .map(|line| line.bytes().fold(
                0 as Letters, | word, c | match c as char {
                    'a' ... 'z' => word | (1 as Letters) << 'z' as u8 - c,
                    _           => !(0 as Letters) }))
        .filter(|word| word.count_ones() <= 7)
        .inspect(|word| words.push(*word))
        .filter(|word| word.count_ones() == 7)
        .collect::<BTreeSet<Letters>>();

    sevens.iter().rev().all(|seven| {
        let scores : [i32;7] = words.iter()
            .filter(|word| **word & !seven == 0)
            .map(|word| (word, if *word == *seven { 3 } else { 1 }))
            .fold([0;7],
                |mut scores, (word, points)| {
                    let mut rest = *seven;
                    for score in &mut scores {
                        let mask = rest & !(rest - 1);
                        rest &= !mask;
                        if (word & mask) != 0 {
                            *score += points } }
                    scores
                });

        let mut buf = String::new();
        let (any, _) = scores.iter().fold((false, *seven),
            |(any, rest), points| {
                let mut this = false;
                let z = match *points {
                    26 ... 32 => { this = true; 'Z' },
                    _         => {             'z' } } as u8;
                let c = z - (rest.trailing_zeros() as u8);
                buf.insert(0, c as char);
                (any | this, rest & rest - 1)
            });
        if any {
            println!("{}", buf) }
        true
    });
} 
