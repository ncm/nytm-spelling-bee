use std::io;
use std::io::prelude::*;
use std::env::args;
use std::fs::File;
use std::io::BufReader;
use std::collections::BTreeSet;
use std::fmt;
// use std::io::BufRead;
// use std::collections::Vec;

fn main() {
    let name = match args().nth(1) {
        Some(n) => n,
        _ => String::from("/usr/share/dict/words")
    };
    let mut f = if name == "-" {
            std::io::stdin().lock().lines()
        } else {
            BufReader::new(File::open(name)
                .ok().expect("file open failed")).lines()
//              .ok().expect(format!("file open failed, {}", name))).lines()
        };

    type Letters = u32;
    let mut words : Vec<Letters> = Vec::new();
    let mut sevens : BTreeSet<Letters> = BTreeSet::new();

    for l in f {
        let line = l.unwrap();
        if line.len() >= 5 {
            let word = 0 as Letters;
            for c in line {
                const one : Letters = 1;
                word = word | match c {
                        'a' ... 'z' => (one << (('z' as i8) - (c as i8))),
                        _ => !(0 as Letters)
                }
            }
            match word.count_ones() {
                7       => { sevens.insert(word); words.push(word)},
                0 ... 6 => words.push(word),
                _    => ()
            }
        }
    }

    for seven in &sevens.iter().rev() {
        let mut scores = [0; 7];
        for word in &words {
            if word & !seven == 0 {
                if word == seven {
                    for mut points in &scores {
                        points += 3;
                    }
                } else {
                    let mut rest = seven;
                    for mut points in &scores {
                        if word & rest & !(rest - 1) {
                            points += 1;
                        }
                        rest &= rest - 1;
                    }
                }
            }
        }
        let mut any = false;
        let mut rest = seven;
        let mut buf : String::new();
        buf = "       ";
        let mut i = 0;
        for mut points in &scores {
            let z = match points { 25 ... 33 => { any = true; 'Z'}, _ => 'z' };
            let c = (z as u8) - (rest.trailing_zeros() as u8);
            buf[6 - i] = c as char;
            rest &= rest - 1; 
            i += 1;
        }

        if any {
            println!("{}", String::new(buf));
        }
    }
}
