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
        .scan((0u32, 0), |state, c| {
            let (word, len) = *state;
            Some(match c as char {
                '\n' => { *state = (0, 0);
                          if len >= 5 { Some(word) } else { None } },
                'a' ... 'z' if len != -1 =>
                    { let word = word | 1 << (25 - (c - ('a' as u8)));
                      *state = if word.count_ones() <= 7
                          { (word, len + 1) } else { (0, -1) }; None },
                _ => { *state = (0, -1); None }
            })
        }).filter_map(|option| option)
        .filter_map(|word| if word.count_ones() == 7
                { *sevens.entry(word).or_insert(0) += 1; None }
            else { Some(word) })
        .collect();

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
