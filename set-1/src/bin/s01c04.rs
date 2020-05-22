use std::fs;
use std::str;

use data_encoding::HEXLOWER;

fn main() -> Result<(), anyhow::Error> {
    let path = format!("{}/../data/s01c04.txt", env!("CARGO_MANIFEST_DIR"));
    let input = fs::read_to_string(path)?;
    let mut best = (0, 0, Vec::new());
    for ln in input.lines() {
        let candidate = HEXLOWER.decode(ln.trim().as_bytes()).unwrap();
        let best_option = set_1::find_key(&candidate);
        if best_option.1 > best.1 {
            best = best_option;
        }
    }
    println!("{}", str::from_utf8(&best.2).unwrap());
    Ok(())
}
