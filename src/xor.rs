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

///
/// create a slice of length L of bytes 
fn cycle_bytes(bytes: &str, L: u32) -> Vec<u8> {
    let iter: Vec<_> = bytes.chars();
    (0..L).zip(&bytes.chars()).map(|b| b.1).collect()
}

#[test]
fn test_cycle_bytes() {
    let expected = b"abcabcabc";
    let actual = cycle_bytes("abc", 9);

    assert_eq!(actual, expected);
}
/// takes a hex string and xors against each char A-Za-z
/// returning the most likely to be valid English
/// 
/// scoring:
///     - create a table C of chars to frequency in English language
///     like this:
///     https://en.wikipedia.org/wiki/Letter_frequency#Relative_frequencies_of_letters_in_the_English_language
///     - reduce the decoded/xor'd bytes B into their own freq table t
///     - for idx, b in B:
///         sum the difference of each key in t to corresponding key in  C
///
///     return the key with the lowest sum (lowest == smaller deviation from the "ideal")
///
///
///
pub fn single_byte(bytes: &str) -> Result<Vec<u8>, ()> {
    // let l: u32 = bytes.len();

    // for ch in LETTERS {
    //     let decoded = hex::decode(bytes)?;
    //     let _key = cycle_bytes(ch, &l);

    //     let cipher = xor_fixed(_key, decoded);
    //     let score = score_cipher(cipher, ch);
    // }
    Ok(vec![1, 2, 3])
}
