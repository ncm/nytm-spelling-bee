use std::io::prelude::*;
use std::{fs, io, env, process};

#[no_mangle] pub extern fn rs_main() {
    let (stdin, filename) = (io::stdin(),
        env::args().nth(1).unwrap_or(String::from("/usr/share/dict/words")));
    let file: Box<Read> = match &filename[..] {
        "-" => Box::new(stdin.lock()),
        _ => Box::new(fs::File::open(&filename).unwrap_or_else(|err| -> _ {
                 writeln!(io::stderr(), "{}: \"{}\"", err, filename).unwrap();
                 process::exit(1);
             }))
    };

    let mut words = Vec::with_capacity(1 << 15);
    let mut sevens = Vec::with_capacity(1 << 16);
    let (mut word, mut len) = (0u32, 0);
    for c in io::BufReader::new(file).bytes().filter_map(Result::ok) {
        if c == b'\n' {
            if len >= 5 {
                if word.count_ones() == 7 {
                        sevens.push((word, 0))
                } else { words.push(word) }
            }
            word = 0; len = 0;
        } else if len != -1 && c >= b'a' && c <= b'z' {
            word |= 1 << (25 - (c - b'a'));
            len = if word.count_ones() <= 7 { len + 1 } else { -1 }
        } else { len = -1 }
    }
    sevens.sort_by(|a,b| b.cmp(a));
    let mut place = 0;
    for i in 0..sevens.len() {
        if sevens[i].0 != sevens[place].0
            { place += 1; sevens[place] = sevens[i]; }
        sevens[place].1 += 1;
    }
    sevens.resize(place + 1, (0,0));

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
                    { 26 ... 32 => { any = true; 'A' }, _ => 'a' };
                *outc = a as u8 + (25 - rest.trailing_zeros()) as u8;
                (any, rest & rest - 1)
            });
        if any
            { sink.write(&out).unwrap(); };
    }
}
#[cfg(not(main))] fn main() { rs_main(); }
