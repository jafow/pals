extern crate hex;
use table;

use std::string::FromUtf8Error;

/// xor_fixed
/// take 2 equal length buffers and return the fixed
/// xor of them
pub fn xor_fixed(buf1: &String, buf2: &String) -> Result<Vec<u8>, hex::FromHexError> {
    assert_eq!(buf1.len(), buf2.len());
    let b1_decoded = hex::decode(buf1.as_bytes())?;
    let b2_decoded = hex::decode(buf2.as_bytes())?;

    let mut res: Vec<u8> = Vec::new();

    for (b1, b2) in b1_decoded.iter().zip(b2_decoded.iter()) {
        res.push(b1 ^ b2);
    }
    Ok(res)
}


#[test]
fn test_xor_fixed() {
    let expected = String::from("746865206b696420646f6e277420706c6179");
    let a1 = String::from("1c0111001f010100061a024b53535009181c");
    let a2 = String::from("686974207468652062756c6c277320657965");
    let actual = xor_fixed(&a1, &a2);

    assert_eq!(hex::encode(actual.unwrap()), expected);
}

#[test]
fn test_xor_err() {
    let mut s1 = String::from("123");
    let mut s2 = String::from("foo");
    let actual = xor_fixed(&s1, &s2);
    assert_eq!(actual, Err(hex::FromHexError::OddLength));
}

/// create a slice of length slice_len of bytes
fn cycle_bytes(slice_len: u8, bytes: &str) -> Vec<&u8> {
    let b = bytes.as_bytes();
    (0..slice_len).zip(b.iter().cycle()).map(|b| b.1).collect()
}

#[test]
fn test_cycle_bytes() {
    let expected: [&u8; 5] = [&b'a', &b'b', &b'c', &b'a', &b'b'];
    let actual = cycle_bytes(5, "abc");

    assert_eq!(actual, expected);
}

fn fill_bytes(slice_len: usize, _char: &u8) -> Vec<u8> {
    let mut filled: Vec<u8> = Vec::new();

    for _i in 0..slice_len {
        filled.push(*_char);
    }
    filled
}

#[test]
fn test_fill_bytes() {
    let bytes_input = "hello";
    let expect0 = vec![b'f', b'f', b'f', b'f', b'f'];
    let actual0 = fill_bytes(bytes_input.len(), &b'f');

    let expect1 = vec![b'a', b'a', b'a', b'a', b'a'];
    let actual1 = fill_bytes(5, &b'a');

    let expect2 = vec![b'z'];
    let actual2 = fill_bytes(1, &b'z');

    assert_eq!(expect0, actual0);
    assert_eq!(expect1, actual1);
    assert_eq!(expect2, actual2);
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
///     return the key with the lowest sum (where lowest is the smaller deviation from the ideal)

pub fn single_byte(bytes: &str) -> Result<Vec<u8>, hex::FromHexError> {
    debug_assert_ne!(bytes.is_empty(), true);
    let _len: usize = bytes.len();
    let decoded = match hex::decode(bytes) {
        Ok(d) => String::from_utf8(d),
        Err(err) => {
            println!("error {:?} decoding bytes {:?}", err, bytes);
            FromUtf8Error
        }
    };

    for ch in table::LETTERS.iter() {
        let _key = fill_bytes(_len, ch);
        let _key_string = String::from_utf8(_key).expect("need a good string here");
        let cipher = xor_fixed(&_key_string, &decoded);
        println!("cipher: {:?}", cipher);
        // let score = score_cipher(cipher, ch);
    }
    Ok(vec![1, 2, 3])
}
