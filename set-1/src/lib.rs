use std::mem;

pub fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b).map(|(a, b)| a ^ b).collect()
}

pub fn find_repeated_key(bytes: &[u8], key_size: usize) -> Vec<u8> {
    let extra = bytes.len() % key_size;
    let mut transposed = vec![0; bytes.len()];
    let block_len = bytes.len() / key_size;
    for (i, &b) in bytes.iter().enumerate() {
        let block = i % key_size;
        let pos = i / key_size;
        let dst = block * block_len + pos + 1 * block.min(extra);
        assert_eq!(transposed[dst], 0);
        transposed[dst] = b;
    }

    (0..key_size)
        .map(|i| {
            let start = i * block_len + (1 * i.min(extra));
            let end = start + block_len + if i < extra { 1 } else { 0 };
            find_byte_key(&transposed[start..end]).0
        })
        .collect::<Vec<_>>()
}

pub fn find_byte_key(input: &[u8]) -> (u8, isize, Vec<u8>) {
    let mut best = (0, 0, Vec::from(input));
    let mut test = Vec::from(input);
    for i in 0u8..=255 {
        test.clear();
        test.extend(input.iter().map(|v| v ^ i));
        let score = score(&test);
        if score > best.1 {
            best.0 = i;
            best.1 = score;
            mem::swap(&mut test, &mut best.2);
        }
    }
    best
}

pub fn score(bytes: &[u8]) -> isize {
    let mut score = 0;
    let mut frequency = [0usize; 26];

    for &b in bytes {
        if let Some(i) = index(b) {
            frequency[i] += 1;
        }

        if b.is_ascii_lowercase() {
            score += 6;
        } else if b.is_ascii_alphabetic() {
            score += 5;
        } else if b.is_ascii_punctuation() {
            score += 1;
        } else if b.is_ascii_whitespace() {
            score += 1;
        } else if !b.is_ascii_graphic() {
            score -= 10;
        } else if b > 128 {
            score -= 10;
        }
    }

    let freqs = frequency.iter().enumerate().filter_map(|(i, freq)| {
        if *freq > 0 {
            Some((-(*freq as i32), i))
        } else {
            None
        }
    });

    let mut amount = 0;
    let mut ranked = [None; 26];
    for (freq, c) in freqs {
        ranked[amount] = Some((freq, c));
        amount += 1;
    }

    for v1 in &ranked[..amount] {
        for v2 in &ranked[..amount] {
            match (v1.unwrap(), v2.unwrap()) {
                ((_, c1), (_, c2)) if c1 >= c2 => {},
                ((f1, c1), (f2, c2)) => {
                    let (e1, e2) = (LETTER_RANKS[c1], LETTER_RANKS[c2]);
                    if e1.cmp(&e2) != f1.cmp(&f2) {
                        score -= 2;
                    } else {
                        score += 1;
                    }

                }
            }
        }
    }

    score
}

fn index(b: u8) -> Option<usize> {
    if b.is_ascii_lowercase() {
        Some((b - 97) as usize)
    } else if b.is_ascii_uppercase() {
        Some((b - 65) as usize)
    } else {
        None
    }
}

const LETTER_RANKS: [u8; 26] = [
    4,  // a
    18, // b
    17, // c
    9,  // d
    0,  // e
    15, // f
    16, // g
    3,  // h
    6,  // i
    22, // j
    20, // k
    10, // l
    13, // m
    7,  // n
    1,  // o
    19, // p
    23, // q
    8,  // r
    5,  // s
    2,  // t
    11, // u
    21, // v
    14, // w
    24, // x
    12, // y
    25, // z
];

pub fn xor_encrypt(key: &[u8], plain: &mut [u8]) {
    plain
        .iter_mut()
        .zip(key.iter().cycle())
        .for_each(|(c, k)| *c ^= k);
}

pub fn find_key_size(bytes: &[u8]) -> usize {
    let mut best = (f64::MAX, 0);
    let mut blocks = Vec::with_capacity(TEST_BLOCKS);
    for key_size in 2..40 {
        blocks.clear();
        for i in 0..TEST_BLOCKS {
            blocks.push(&bytes[i * key_size..(i + 1) * key_size]);
        }

        let mut diffs = 0;
        for outer in 0..(TEST_BLOCKS - 1) {
            for inner in (outer + 1)..TEST_BLOCKS {
                diffs += distance(blocks[outer], blocks[inner]);
            }
        }

        // 6 is the number of combinations for selecting 2 different blocks out of 4
        let normalized = (diffs as f64) / ((key_size * 6) as f64);
        if normalized < best.0 {
            best = (normalized, key_size);
        }
    }

    best.1
}

pub fn distance(a: &[u8], b: &[u8]) -> usize {
    assert_eq!(a.len(), b.len());
    let mut res = 0;
    for (&x, &y) in a.iter().zip(b) {
        let combined = x ^ y;
        for i in 0..8 {
            if (combined >> i) & 1 > 0 {
                res += 1;
            }
        }
    }
    res
}

const TEST_BLOCKS: usize = 4;

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

    #[test]
    fn s01c05() {
        let mut input = Vec::from(
            &b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"[..],
        );
        super::xor_encrypt(b"ICE", &mut input);
        let result = HEXLOWER.encode(&input);
        assert_eq!(
            result,
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272\
             a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
        );
    }

    #[test]
    fn distance() {
        assert_eq!(super::distance(b"this is a test", b"wokka wokka!!!"), 37);
    }

    #[test]
    fn frequency() {
        assert_eq!(super::score(b"foo"), 18);
    }
}
