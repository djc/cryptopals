use std::mem;

pub fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b).map(|(a, b)| a ^ b).collect()
}

pub fn find_key(input: &[u8]) -> (isize, Vec<u8>) {
    let mut best = (0, Vec::from(input));
    let mut test = Vec::from(input);
    for i in 0u8..=255 {
        test.clear();
        test.extend(input.iter().map(|v| v ^ i));
        let score = score(&test);
        if score > best.0 {
            best.0 = score;
            mem::swap(&mut test, &mut best.1);
        }
    }
    best
}

pub fn score(bytes: &[u8]) -> isize {
    let mut score = 0;
    for &b in bytes {
        if b.is_ascii_alphabetic() {
            score += 5;
        } else if b.is_ascii_punctuation() {
            score += 0;
        } else if b.is_ascii_whitespace() {
            score += 2;
        } else if !b.is_ascii_graphic() {
            score -= 3;
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use data_encoding::{BASE64, HEXLOWER};

    #[test]
    fn s01c01() {
        let input = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let bytes = HEXLOWER.decode(input).unwrap();
        let b64 = BASE64.encode(&bytes);
        assert_eq!(b64, output);
    }

    #[test]
    fn s01c02() {
        let a = HEXLOWER
            .decode(b"1c0111001f010100061a024b53535009181c")
            .unwrap();
        let b = HEXLOWER
            .decode(b"686974207468652062756c6c277320657965")
            .unwrap();
        let res = HEXLOWER.encode(&super::xor(&a, &b));
        assert_eq!(res, "746865206b696420646f6e277420706c6179");
    }
}
