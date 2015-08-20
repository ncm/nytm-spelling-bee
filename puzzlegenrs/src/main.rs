use std::io;
use std::io::prelude::*;
use std::env::args;
use std::fs::File;
use std::collections::BTreeSet;
use std::io::BufRead;
// use std::collections::Vec;

use std::intrinsics::ctpop32;
use std::intrinsics::cttz32;

fn main() {
    // let name = match args().nth(1) {
    //     Some(n) => n,
    //     _ => String::from("/usr/share/dict/words")
    // };
    // let mut fs = if name == "-" { std::io::stdin } else { File::open(name) };
    // fs.ok().expect("file open failed, {}", name);
    let fs = std::io::stdin;

    type Letters = u32;
    let mut words : Vec<Letters> = Vec::new();
    let mut sevens : BTreeSet<Letters> = BTreeSet::new();

    for line in fs {
        if line.len() >= 5 {
            let word = 0 as Letters;
            for c in line {
                const one : Letters = 1;
                word = word | match c {
                        'a' ... 'z' => (one << (('z' as i8) - (c as i8))),
                        _ => !(0 as Letters)
                }
            }
            match unsafe { ctpop32(word) } {
                7       => { sevens.insert(word); words.push(word)},
                0 ... 6 => words.push(word),
                _    => ()
            }
        }
    }

    for seven in &sevens.reverse() {
        let mut scores  = [0; 7];
        for word in &words {
            if word & !seven == 0 {
                if word == seven {
                    scores.map_in_place(|e| e + 3);
                } else {
                    let mut rest = seven;
                    for mut points in &scores {
                        if word & rest & -rest {
                            points = points + 1;
                        }
                        rest = rest & !-rest ;
                    }
                }
            }
        }
        let mut any = false;
        let mut rest = seven;
        let mut buf : [char; 7];
        let mut i = 0;
        for points in &scores {
            let z = match points { 25 ... 33 => { any = true; 'Z'}, _ => 'z' };
            let c = (z as u8) - (unsafe { cttz32(rest & -rest) } as u8);
            buf[6 - i] = c as char;
            rest = rest & !-rest; 
            i = i + 1;
        }

        if any {
            println!("{}", buf);
        }
    }
}
