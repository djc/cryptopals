use std::mem;

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

const TEST_BLOCKS: usize = 4;

pub fn distance(a: &[u8], b: &[u8]) -> usize {
    assert_eq!(a.len(), b.len());
    a.iter()
        .zip(b)
        .map(|(&x, &y)| (x ^ y).count_ones())
        .sum::<u32>() as usize
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

pub fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b).map(|(a, b)| a ^ b).collect()
}

pub fn repeated_xor(key: &[u8], plain: &mut [u8]) {
    plain
        .iter_mut()
        .zip(key.iter().cycle())
        .for_each(|(c, k)| *c ^= k);
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

    #[test]
    fn s01c05() {
        let mut input = Vec::from(
            &b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"[..],
        );
        super::repeated_xor(b"ICE", &mut input);
        let result = HEXLOWER.encode(&input);
        assert_eq!(
            result,
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272\
             a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
        );
    }
}
