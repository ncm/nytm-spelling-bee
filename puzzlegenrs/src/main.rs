// use std::io;
use std::io::prelude::*;
// use std::env::args;
// use std::fs::File;
// use std::io::BufReader;
use std::collections::BTreeSet;
// use std::collections::String;

// use std::fmt;
// use std::io::BufRead;
// use std::collections::Vec;

fn main() {
    // let name = match args().nth(1) {
    //     Some(n) => n,
    //     _ => String::from("/usr/share/dict/words")
    // };
    // let mut &f = if name == "-" {
    //         std::io::stdin().lock().lines()
    //     } else {
    //         BufReader::new(File::open(name)
    //             .ok().expect("file open failed")).lines()
//              .ok().expect(format!("file open failed, {}", name))).lines()
    //     };

    let input = std::io::stdin();
    let mut f = input.lock();

    type Letters = u32;
    let mut words : Vec<Letters> = Vec::new();
    let mut sevens : BTreeSet<Letters> = BTreeSet::new();

    loop {
        let mut line : Vec<u8> = Vec::new();
	let len = match f.read_until(10u8, &mut line) {
            Ok(l) => l,
            _ => break };
	if len == 0 {
	    break;
        }
        if len > 5 {
            let mut word = 0 as Letters;
            for c in line {
                const ONE : Letters = 1;
                match c as char {
                    'a' ... 'z' => word |= ONE << (('z' as i8) - (c as i8)),
		    '\n' => break,
                     _ => { word = !0u32; break }
                };
            }
            match word.count_ones() {
                7       => { sevens.insert(word); words.push(word)},
                0 ... 6 => words.push(word),
                _    => ()
            }
        }
    }
    println!("{} {}", words.len(), sevens.len());

//    for seven in &sevens.iter().rev() {
//        let mut scores = [0; 7];
//        for word in &words {
//            if word & !seven == 0 {
//                if word == seven {
//                    for mut points in &scores {
//                        points += 3;
//                    }
//                } else {
//                    let mut rest = seven;
//                    for mut points in &scores {
//                        if word & rest & !(rest - 1) {
//                            points += 1;
//                        }
//                        rest &= rest - 1;
//                    }
//                }
//            }
//        }
//        let mut any = false;
//        let mut rest = seven;
//        let mut buf : String = String::new();
//        buf = "       ";
//        let mut i = 0;
//        for mut points in &scores {
//            let z = match points { 25 ... 33 => { any = true; 'Z'}, _ => 'z' };
//            let c = (z as u8) - (rest.trailing_zeros() as u8);
//            buf[6 - i] = c as char;
//            rest &= rest - 1;
//            i += 1;
//        }
//
//        if any {
//            println!("{}", String::new(buf));
//        }
//    }
}
