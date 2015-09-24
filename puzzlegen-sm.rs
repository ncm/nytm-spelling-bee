use std::io::prelude::*;
use std::{fs,io,env};
use std::collections::BTreeMap;

const WORDS_FILE : &'static str = "/usr/share/dict/words";
type Letters = u32;
const A : Letters = 1 << 25;

#[no_mangle] pub extern fn sm_rs_main() {
    let name = env::args().nth(1).unwrap_or(String::from(WORDS_FILE));
    let stdin = io::stdin();
    let file : Box<io::Read> = match &*name {
        "-" => Box::new(stdin.lock()),
        _   => Box::new(fs::File::open(name).ok().expect("file open failed"))
    };

    let mut sevens : BTreeMap<Letters,u16> = BTreeMap::new();
    let words : Vec<_> = io::BufReader::new(file).bytes()
        .filter_map(|resultc| resultc.ok())
        .scan((0 as Letters, 0), |&mut (ref mut word, ref mut len), c|
            Some(match (c as char, *len) {
                ('\n', -1 ... 4) => { *word = 0; *len = 0; None },
                ('\n', _) => { let w = *word; *word = 0; *len = 0;  Some(w) },
                (_, -1) => None,
                ('a' ... 'z', _) => {
                    *word |= A >> c - ('a' as u8);
                    if word.count_ones() <= 7
                          { *len += 1; None }
                    else { *len = -1; None }
                },
                (_, _) => { *len = -1; None }
            })
        ).filter_map(|option| option)
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
                *out = (a as u8) + (25u32 - rest.trailing_zeros()) as u8;
                (any, rest & rest - 1)
            });
         if any
              { sink.write(&out).unwrap(); };
    }
}
#[cfg(not(main))] fn main() { sm_rs_main(); }
