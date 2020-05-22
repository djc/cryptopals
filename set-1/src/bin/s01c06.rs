use std::fs;
use std::str;

use data_encoding::BASE64;

fn main() -> Result<(), anyhow::Error> {
    let path = format!("{}/../data/s01c06.txt", env!("CARGO_MANIFEST_DIR"));
    let input = fs::read_to_string(path)?.replace("\n", "");
    let mut bytes = BASE64.decode(input.trim().as_bytes())?;
    let key_size = set_1::find_key_size(&bytes);

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

    let key = (0..key_size)
        .map(|i| {
            let start = i * block_len + (1 * i.min(extra));
            let end = start + block_len + if i < extra { 1 } else { 0 };
            set_1::find_key(&transposed[start..end]).0
        })
        .collect::<Vec<_>>();

    set_1::xor_encrypt(&key, &mut bytes);
    println!("{}", str::from_utf8(&bytes)?);

    Ok(())
}
