use std::collections::HashMap;
use std::fs;

use data_encoding::HEXLOWER;

fn main() -> Result<(), anyhow::Error> {
    let path = format!("{}/../data/s01c08.txt", env!("CARGO_MANIFEST_DIR"));
    let input = fs::read_to_string(path)?;
    let mut best = (0, 0, Vec::new());

    for (i, ln) in input.lines().enumerate() {
        let candidate = HEXLOWER.decode(ln.trim().as_bytes()).unwrap();

        let mut map = HashMap::new();
        for block in candidate.chunks(AES_BLOCK_SIZE) {
            let freq = map.entry(block).or_insert(-1);
            *freq += 1;
        }
        let score = map.values().sum();

        if score > best.0 {
            best.0 = score;
            best.1 = i;
            best.2 = candidate;
        }
    }

    println!("{}", HEXLOWER.encode(&best.2));
    Ok(())
}

const AES_BLOCK_SIZE: usize = 16;
