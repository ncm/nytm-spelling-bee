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

    let mut sevens = Vec::with_capacity(1 << 14);
    let mut words = Vec::with_capacity(1 << 15);
    let (mut word, mut len, mut ones) = (0u32, 0, 0);
    for c in io::BufReader::new(file).bytes().filter_map(Result::ok) {
        if c == b'\n' {
            if len >= 5 && ones <= 7
                { if ones == 7 { sevens.push(word) } else { words.push(word) } }
            word = 0; len = 0; ones = 0;
        } else if ones != 8 && c >= b'a' && c <= b'z' {
            word |= 1 << (25 - (c - b'a')); len += 1; ones = word.count_ones()
        } else { ones = 8 }
    }

    sevens.sort();
    let (mut count, mut prev, mut counts) = (0, 0, vec![0u16; sevens.len()]);
    if !sevens.is_empty() { prev = sevens[0]; counts[0] = 3 }
    for i in 1..sevens.len() {
        if prev != sevens[i]
            { count += 1; prev = sevens[i]; sevens[count] = prev; }
        counts[count] += 3;
    }

    let stdout = io::stdout();
    let mut sink = io::BufWriter::new(stdout.lock());
    for count in (0..(count + 1)).rev() {
        let seven = sevens[count];
        let (mut rest, mut bits) = (seven, [0u16;7]);
        for place in (0..7).rev()
            { bits[place] = rest.trailing_zeros() as u16; rest &= rest - 1 }
        let scores = words.iter()
            .filter(|&word| word & !seven == 0)
            .fold([counts[count];7], |mut scores, &word| {
                for place in 0..7
                     { scores[place] += ((word >> bits[place]) & 1) as u16; }
                scores
            });

        let (mut any, mut out) = (false, *b".......\n");
        for place in 0..7 {
            let a = match scores[place]
               { 26 ... 32 => { any = true; b'A' }, _ => b'a' };
            out[place] = a + (25 - bits[place]) as u8
        }
        if any
            { sink.write(&out).unwrap(); };
    }
}
#[cfg(not(main))] fn main() { rs_main(); }
