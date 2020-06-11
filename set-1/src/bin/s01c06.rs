use std::fs;
use std::str;

use data_encoding::BASE64;

fn main() -> Result<(), anyhow::Error> {
    let path = format!("{}/../data/s01c06.txt", env!("CARGO_MANIFEST_DIR"));
    let input = fs::read_to_string(path)?.replace("\n", "");
    let mut bytes = BASE64.decode(input.trim().as_bytes())?;

    let key_size = set_1::find_key_size(&bytes);
    let key = set_1::find_repeated_key(&bytes, key_size);
    set_1::repeated_xor(&key, &mut bytes);
    println!("{}", str::from_utf8(&bytes)?);

    Ok(())
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

const TEST_BLOCKS: usize = 4;

pub fn distance(a: &[u8], b: &[u8]) -> usize {
    assert_eq!(a.len(), b.len());
    a.iter()
        .zip(b)
        .map(|(&x, &y)| (x ^ y).count_ones())
        .sum::<u32>() as usize
}

#[cfg(test)]
mod tests {
    #[test]
    fn distance() {
        assert_eq!(super::distance(b"this is a test", b"wokka wokka!!!"), 37);
    }
}
