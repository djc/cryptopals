use data_encoding::HEXLOWER;

fn main() {
    let bytes = HEXLOWER.decode(INPUT.as_bytes()).unwrap();
    let combine = HEXLOWER.decode(COMBINE.as_bytes()).unwrap();
    let combined = bytes
        .iter()
        .zip(combine.iter())
        .map(|(x, y)| x ^ y)
        .collect::<Vec<_>>();
    let hex = HEXLOWER.encode(&combined);
    println!("hex: {}", hex);
    assert_eq!(hex, OUTPUT);
}

const INPUT: &str = "1c0111001f010100061a024b53535009181c";
const COMBINE: &str = "686974207468652062756c6c277320657965";
const OUTPUT: &str = "746865206b696420646f6e277420706c6179";
