extern crate hex;

/// xor_fixed
/// take 2 equal length buffers and return the fixed
/// xor of them
pub fn xor_fixed(buf1: &[u8], buf2: &[u8]) -> Result<Vec<u8>, ()> {
    let mut res: Vec<u8> = Vec::new();

    for (b1, b2) in buf1.iter().zip(buf2.iter()) {
        res.push(b1 ^ b2);
    }
    Ok(res)
}

#[test]
fn test_xor_fixed() {
    let res = xor_fixed(b"1c0111001f010100061a024b53535009181c",
                        b"686974207468652062756c6c277320657965");

    assert_eq!(res, Ok(b"746865206b696420646f6e277420706c6179".to_vec()));
}
