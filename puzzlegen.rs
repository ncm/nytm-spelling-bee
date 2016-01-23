use std::io::prelude::*;
use std::{fs,io,env,process};

#[no_mangle] pub extern fn rs_main() {
    let filename = env::args().nth(1).unwrap_or(
        String::from("/usr/share/dict/words"));
    let stdin = io::stdin();
    let file : Box<io::Read> = match &*filename {
        "-" => Box::new(stdin.lock()),
        _ => match fs::File::open(&*filename) {
            Ok(f) => Box::new(f), Err(x) => {
               writeln!(io::stderr(), "{}: \"{}\"", x, filename).unwrap();
               process::exit(1)
            }
        }
    };

    let (mut sevens, mut words) = (Vec::new(), Vec::new());
    let (mut word, mut len) = (0u32, 0);
    for c in io::BufReader::new(file).bytes().filter_map(Result::ok) {
        if c == b'\n' {
            if len >= 5 {
                if word.count_ones() == 7 {
                        sevens.push(word)
                } else { words.push(word) }
            }
            word = 0; len = 0;
        } else if len != -1 && c >= b'a' && c <= b'z' {
            word |= 1 << (25 - (c - b'a'));
            len = if word.count_ones() <= 7 { len + 1 } else { -1 }
        } else { len = -1 }
    }
    sevens.sort();
    let sevens : Vec<_> = sevens.iter().skip(1).chain([0].iter())
        .scan((sevens[0], 1u16), |&mut (ref mut prev, ref mut count), &seven|
            if seven != *prev {
               let pair = (*prev, *count); *prev = seven; *count = 1;
               Some(Some(pair))
            } else { *count += 1; Some(None) }
         ).filter_map(|pair| pair).collect();

    let stdout = io::stdout();
    let mut sink = io::BufWriter::new(stdout.lock());
    for &(seven, count) in sevens.iter().rev() {
        let scores = words.iter().filter(|&word| word & !seven == 0)
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
