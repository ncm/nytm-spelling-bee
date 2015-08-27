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
    let mut input = std::io::BufReader::new(file);

    type Letters = u32;
    let mut words : Vec<Letters> = Vec::new();
    let mut sevens : BTreeSet<Letters> = BTreeSet::new();

    let mut word : Letters = 0;
    let mut length : i32 = 0;
    loop {
        let mut buf = [0u8, 1];
        match input.read(&mut buf) { Ok(1) => (), _ => break };
        match (buf[0] as char, length) {
            ('\n', -1 ... 5) => length = 0,
            ('\n', _) => { words.push(word); length = 0;
                 if word.count_ones() == 7 {
                     sevens.insert(word); }
                 word = 0; },
            (_, -1) => (), 
            ('a' ... 'z', _) => { length += 1;
                 word |= 1u32 << (('z' as u8) - buf[0]) },
            (_, _)   => { word = 0; length = 0; }
        }
    }

    let mut line : Vec<u8> = Vec::new();
    loop {
        line.clear();
	let len = match input.read_until('\n' as u8, &mut line) {
            Ok(0) => break, Ok(l) => l, _ => break };
        if len > 5 {
            let mut word = 0;
            for c in &line {
                match *c as char {
                    'a' ... 'z' => word |= 1u32 << (('z' as u8) - *c),
		    '\n' => break,
                     _ => { word = !0u32; break }
                };
            }
            match word.count_ones() {
                7       => { sevens.insert(word); words.push(word); },
                0 ... 6 => {                      words.push(word); },
                _    => ()
            }
        }
    }

    for seven in sevens.iter().rev() {
        let mut scores = [0; 7];
        for word in words.iter() {
            if *word & !*seven == 0 {
                let points = if *word == *seven { 3 } else { 1 } ;
                let mut rest : Letters = *seven;
                for score in &mut scores {
                    if (*word & rest & !(rest - 1)) != 0 {
                         *score += points;
                    }
                    rest &= rest - 1;
                }
            }
        }
        let mut any = false;
        let mut rest : Letters = *seven;
        let mut buf : String = String::new();
        for points in &scores {
            let z = match *points {
                26 ... 32 => { any = true; 'Z' }, _ => 'z' };
            let c = (z as u8) - (rest.trailing_zeros() as u8);
            buf.insert(0, c as char);
            rest &= rest - 1;
        }
        if any {
            println!("{}", buf);
        }
    }
} 
