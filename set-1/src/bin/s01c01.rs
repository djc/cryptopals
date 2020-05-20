use data_encoding::{BASE64, HEXLOWER};

fn main() {
    let bytes = HEXLOWER.decode(INPUT).unwrap();
    let b64 = BASE64.encode(&bytes);
    println!("base64: {}", b64);
    assert_eq!(b64, OUTPUT);
}

const INPUT: &[u8] = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
const OUTPUT: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
