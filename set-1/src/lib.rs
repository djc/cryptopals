pub fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b).map(|(a, b)| a ^ b).collect()
}

#[cfg(test)]
mod tests {
    use data_encoding::HEXLOWER;

    #[test]
    fn challenge_1_2() {
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
