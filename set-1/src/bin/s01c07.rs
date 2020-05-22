use std::fs;
use std::str;

use aes::Aes128;
use block_modes::{BlockMode, Ecb, block_padding::Pkcs7};
use data_encoding::BASE64;

fn main() -> Result<(), anyhow::Error> {
    let path = format!("{}/../data/s01c07.txt", env!("CARGO_MANIFEST_DIR"));
    let input = fs::read_to_string(path)?.replace("\n", "");
    let mut bytes = BASE64.decode(input.trim().as_bytes())?;

    let cipher = Ecb::<Aes128, Pkcs7>::new_var(KEY, Default::default()).unwrap();
    let plain = cipher.decrypt(&mut bytes).unwrap();
    println!("{}", str::from_utf8(plain)?);

    Ok(())
}

const KEY: &[u8; 16] = b"YELLOW SUBMARINE";
