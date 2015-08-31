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

    for seven in sevens.iter().rev() {
        let mut scores = [0; 7];
        for word in words.iter() {
            if *word & !*seven == 0 {
                let points = if *word == *seven { 3 } else { 1 } ;
                let mut rest = *seven;
                for score in &mut scores {
                    if (*word & rest & !(rest - 1)) != 0 {
                         *score += points }
                    rest &= rest - 1
                }
            }
        }
        let mut any = false;
        let mut rest = *seven;
        let mut buf = String::new();
        for points in &scores {
            let z = match *points {
                26 ... 32 => { any = true; 'Z' },
                _         => {             'z' } } as u8;
            let c = z - (rest.trailing_zeros() as u8);
            buf.insert(0, c as char);
            rest &= rest - 1
        }
        if any {
            println!("{}", buf) }
    }
} 
