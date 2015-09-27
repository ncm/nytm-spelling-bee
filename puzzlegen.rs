use std::io::prelude::*;
use std::{fs,io,env};
use std::collections::BTreeMap;

#[no_mangle] pub extern fn rs_main() {
    let name = env::args().nth(1).unwrap_or(
        String::from("/usr/share/dict/words"));
    let stdin = io::stdin();
    let file : Box<io::Read> = match &*name {
        "-" => Box::new(stdin.lock()),
        _   => Box::new(fs::File::open(name).ok().expect("file open failed"))
    };

    let mut sevens : BTreeMap<u32,u16> = BTreeMap::new();
    let words : Vec<_> = io::BufReader::new(file).bytes()
        .filter_map(|resultc| resultc.ok())
        .scan((0u32, 0), |&mut (ref mut word, ref mut len), c| {
            match c as char {
                '\n' => {
                   let (w, l) = (*word, *len); *word = 0; *len = 0;
                   if l >= 5
                       { return Some(Some(w)) }
                }, 'a' ... 'z' if *len != -1 => {
                   let w = *word | 1 << (25 - (c - ('a' as u8)));
                   if w.count_ones() <= 7
                      { *word = w; *len += 1 }
                   else { *len = -1 }
                }, _ =>  { *len = -1 }
            }; Some(None)
        }).filter_map(|option| option).filter_map(|word|
            if word.count_ones() < 7
                 { Some(word) }
            else { *sevens.entry(word).or_insert(0) += 1; None }
        ).collect();

    let stdout = io::stdout();
    let mut sink = io::BufWriter::new(stdout.lock());
    for (&seven,&count) in sevens.iter().rev() {
        let scores = words.iter().map(|&word| word)
            .filter(|&word| word & !seven == 0)
            .fold([0u16;7], |mut scores, word| {
                scores.iter_mut().fold(seven, |rest, score| {
                   if word & rest & !(rest - 1) != 0
                       { *score += 1 }
                   rest & rest - 1
                });
                scores
            });

        let mut out = [0, 0, 0, 0, 0, 0, 0, '\n' as u8];
        let (any, _) = scores.iter().zip(out.iter_mut().rev().skip(1))
            .fold((false, seven), |(mut any, rest), (&score, out)| {
                let a = match score + count * 3
                    { 26 ... 32 => { any = true; 'A' }, _ => 'a' };
                *out = (a as u8) + (25 - rest.trailing_zeros()) as u8;
                (any, rest & rest - 1)
            });
        if any
            { sink.write(&out).unwrap(); };
    }
}
#[cfg(not(main))] fn main() { rs_main(); }
