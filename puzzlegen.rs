use std::io::prelude::*;
use std::{fs, io, env, process};

#[no_mangle] pub extern fn rs_main() {
    let fname = env::args().nth(1).unwrap_or("/usr/share/dict/words".into());
    let stdin = io::stdin();
    let file: Box<Read> = match &fname[..] {
        "-" => Box::new(stdin.lock()),
        _ => Box::new(fs::File::open(&fname).unwrap_or_else(|err| {
                 writeln!(io::stderr(), "{}: \"{}\"", err, fname).unwrap();
                 process::exit(1);
             }))
    };

    let mut words = Vec::with_capacity(1 << 15);
    let mut sevens = Vec::with_capacity(1 << 16);
    let (mut word, mut len, mut skip) = (0u32, 0, false);
    for c in io::BufReader::new(file).bytes().filter_map(Result::ok) {
        if c == b'\n' {
            if !skip && len >= 5 {
                if word.count_ones() == 7 {
                        sevens.push((word, 0))
                } else { words.push(word) }
            }
            word = 0; len = 0; skip = false;
        } else if !skip && c >= b'a' && c <= b'z' {
            word |= 1 << (25 - (c - b'a'));
            if word.count_ones() <= 7 { len += 1 } else { skip = true }
        } else { skip = true }
    }

    sevens.sort_by(|a, b| b.0.cmp(&a.0));
    let mut place = 0;
    for i in 0..sevens.len() {
        if sevens[place].0 != sevens[i].0
            { place += 1; sevens[place] = sevens[i]; }
        sevens[place].1 += 1
    }
    if !sevens.is_empty() { sevens.resize(place + 1, (0,0)) }

    let stdout = io::stdout();
    let mut sink = io::BufWriter::new(stdout.lock());
    for &(seven, count) in sevens.iter() {
        let scores = words.iter()
            .filter(|&word| word & !seven == 0)
            .fold([0u16;7], |mut scores, word| {
                scores.iter_mut().fold(seven, |rest, score| {
                   if word & rest & !(rest - 1) != 0
                       { *score += 1 }
                   rest & rest - 1
                });
                scores
            });

        let mut out = *b".......\n";
        let (any, _) = scores.iter().zip(out.iter_mut().rev().skip(1))
            .fold((false, seven), |(mut any, rest), (&score, outc)| {
                let a = match score + 3 * count
                    { 26 ... 32 => { any = true; b'A' }, _ => b'a' };
                *outc = a + (25 - rest.trailing_zeros()) as u8;
                (any, rest & rest - 1)
            });
        if any
            { sink.write(&out).unwrap(); };
    }
}
#[cfg(not(main))] fn main() { rs_main(); }
