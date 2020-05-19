use std::str;

use data_encoding::HEXLOWER;

fn main() {
    let bytes = HEXLOWER.decode(INPUT.as_bytes()).unwrap();
    let mut best = (0, bytes.clone());
    for i in 0u8..=255 {
        let candidate = bytes.iter().map(|v| v ^ i).collect::<Vec<_>>();
        let count = candidate.iter().filter(|v| (*v).is_ascii_alphabetic()).count();
        if count > best.0 {
            best = (count, candidate);
        }
    }
    println!("{}", str::from_utf8(&best.1).unwrap());
}

const INPUT: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
