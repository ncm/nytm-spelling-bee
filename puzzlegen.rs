use std::io::prelude::*;
use std::{fs, io, env, process};

#[no_mangle] pub extern fn rs_main() {
    let fname = &*env::args().nth(1).unwrap_or("/usr/share/dict/words".into());
    let stdin = io::stdin();
    let file: Box<Read> = match fname {
        "-" => Box::new(stdin.lock()),
        _ => Box::new(fs::File::open(fname).unwrap_or_else(|err| {
                 writeln!(io::stderr(), "{}: \"{}\"", err, fname).unwrap();
                 process::exit(1);
             }))
    };

    let mut words = Vec::with_capacity(1 << 15);
    let mut sevens = Vec::with_capacity(1 << 14);
    let (mut word, mut len, mut skip) = (0u32, 0, false);
    for c in io::BufReader::new(file).bytes().filter_map(Result::ok) {
        if c == b'\n' {
            if !skip && len >= 5 {
                if word.count_ones() == 7
                     { sevens.push(word) }
                else { words.push(word) }
            }
            word = 0; len = 0; skip = false;
        } else if !skip && c >= b'a' && c <= b'z' {
            word |= 1 << (25 - (c - b'a'));
            if word.count_ones() <= 7 { len += 1 } else { skip = true }
        } else { skip = true }
    }

    sevens.sort();
    let (mut prev, mut count, mut counts) = (0, !0, vec![0; sevens.len()]);
    for i in 0..sevens.len() {
        if prev != sevens[i]
            { count += 1; prev = sevens[i]; sevens[count] = prev; }
        counts[count] += 3;
    }

    let stdout = io::stdout();
    let mut sink = io::BufWriter::new(stdout.lock());
    for count in (0..(count + 1)).rev() {
        let seven = sevens[count];
        let scores = words.iter()
            .filter(|&word| word & !seven == 0)
            .fold([0;7], |mut scores, word| {
                scores.iter_mut().fold(seven, |rest, score| {
                   if word & rest & !(rest - 1) != 0
                       { *score += 1 }
                   rest & rest - 1
                });
                scores
            });

        let threes = counts[count];
        let (mut any, mut rest, mut out) = (false, seven, *b".......\n");
        for place in 0..7 {
            let a = match scores[place] + threes
               { 26 ... 32 => { any = true; b'A' }, _ => b'a' };
            out[6 - place] = a + (25 - rest.trailing_zeros()) as u8;
            rest &= rest - 1
        }
        if any
            { sink.write(&out).unwrap(); };
    }
}
#[cfg(not(main))] fn main() { rs_main(); }
