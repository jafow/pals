extern crate hex;

/// xor_fixed
/// take 2 equal length buffers and return the fixed
/// xor of them
pub fn xor_fixed(buf1: &str, buf2: &str) -> Result<Vec<u8>, hex::FromHexError> {
    assert_eq!(buf1.len(), buf2.len());
    let b1_decoded = hex::decode(buf1)?;
    let b2_decoded = hex::decode(buf2)?;

    let mut res: Vec<u8> = Vec::new();

    for (b1, b2) in b1_decoded.iter().zip(b2_decoded.iter()) {
        res.push(b1 ^ b2);
    }
    Ok(res)
}


#[test]
fn test_xor_fixed() {
    let expected = String::from("746865206b696420646f6e277420706c6179");
    let actual = xor_fixed("1c0111001f010100061a024b53535009181c",
                        "686974207468652062756c6c277320657965");

    assert_eq!(hex::encode(actual.unwrap()), expected);
}

#[test]
fn test_xor_err() {
    let actual = xor_fixed("123",
                        "foo");
    assert_eq!(actual, Err(hex::FromHexError::OddLength));
}
