use std::str;

use data_encoding::HEXLOWER;

fn main() {
    let bytes = HEXLOWER.decode(INPUT.as_bytes()).unwrap();
    let best = set_1::find_key(&bytes);
    println!("{}", str::from_utf8(&best.1).unwrap());
}

const INPUT: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
