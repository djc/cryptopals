use std::str;

use data_encoding::HEXLOWER;

fn main() {
    let bytes = HEXLOWER.decode(INPUT.as_bytes()).unwrap();
    let mut best = (0, bytes.clone());
    for i in 0u8..=255 {
        let candidate = bytes.iter().map(|v| v ^ i).collect::<Vec<_>>();
        let score = set_1::score(&candidate);
        if score > best.0 {
            best = (score, candidate);
        }
    }
    println!("{}", str::from_utf8(&best.1).unwrap());
}

const INPUT: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
