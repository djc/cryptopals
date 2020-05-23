use std::fs;
use std::str;

use data_encoding::BASE64;

fn main() -> Result<(), anyhow::Error> {
    let path = format!("{}/../data/s01c06.txt", env!("CARGO_MANIFEST_DIR"));
    let input = fs::read_to_string(path)?.replace("\n", "");
    let mut bytes = BASE64.decode(input.trim().as_bytes())?;

    let key_size = set_1::find_key_size(&bytes);
    let key = set_1::find_repeated_key(&bytes, key_size);
    set_1::xor_encrypt(&key, &mut bytes);
    println!("{}", str::from_utf8(&bytes)?);

    Ok(())
}
