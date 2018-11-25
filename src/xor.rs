extern crate hex;
use table;

use std::collections::HashMap;

struct FreqScore {
    id: u8,
    score: f32
}

struct Freq {
    raw: u8,
    pct: f32
}


/// xor_fixed
/// take 2 equal length buffers and return the fixed
/// xor of them
pub fn xor_fixed(buf1: &[u8], buf2: &[u8]) -> Result<Vec<u8>, hex::FromHexError> {
    // assert_eq!(buf1.len(), buf2.len());
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
    let a1: &[u8; 36] = b"1c0111001f010100061a024b53535009181c";
    let a2: &[u8; 36] = b"686974207468652062756c6c277320657965";
    let actual = xor_fixed(a1, a2);

    assert_eq!(hex::encode(actual.unwrap()), expected);
}

#[test]
fn test_xor_err() {
    let s1 = b"123";
    let s2 = b"foo";
    let actual = xor_fixed(s1, s2);
    assert_eq!(actual, Err(hex::FromHexError::OddLength));
}

/// create a slice of length slice_len of bytes
fn cycle_bytes(slice_len: u8, bytes: &[u8]) -> Vec<&u8> {
    (0..slice_len).zip(bytes.iter().cycle()).map(|b| b.1).collect()
}

#[test]
fn test_cycle_bytes() {
    let expected: [&u8; 5] = [&97, &98, &99, &97, &98];
    let actual = cycle_bytes(5, b"abc");

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
///     - create a table T of chars to frequency in English language
///     like this:
///     https://en.wikipedia.org/wiki/Letter_frequency#Relative_frequencies_of_letters_in_the_English_language
///     - reduce the decoded/xor'd bytes B into their own freq table t
///     - for idx, b in B:
///         sum the difference of each key in t to corresponding key in T
///     return the key with the lowest sum (where lowest is the smaller deviation from the ideal)

pub fn single_byte(bytes: &str) -> u8 {
    debug_assert_ne!(bytes.is_empty(), true);
    let _len: usize = bytes.len();
    let mut min_score: FreqScore = FreqScore {id: 0, score: 100.0};

    for ch in table::LETTERS.iter() {
        // fill up a buffer with a byte
        let _key = hex::encode(fill_bytes(_len, ch));
        // xor ciphertext against `_key`
        let cipher = xor_fixed(&_key.into_bytes(), &bytes.as_bytes()).expect("xor fixed");

        let cipher_table: HashMap<u8, Freq> = freq_table(&cipher);

        let score: FreqScore = score_cipher(cipher_table, *ch);
        if score.score <= min_score.score {
            min_score = score;
        }
    }
    min_score.id
}

#[test]
fn test_single_byte() {
    let actual = single_byte("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let expected = 88;

    assert_eq!(actual, expected);
}

/// creates a frequency table of chars in a bytearray
fn freq_table(bytes: &[u8]) -> HashMap<u8, Freq> {
    let mut table: HashMap<u8, Freq> = HashMap::new();
    for &c in bytes.iter() {
        let mut fq = Freq { raw: 1, pct: 0.0 };
        match table.get(&c) {
            Some(v) => fq.raw += v.raw,
            None => fq.raw += 0
        };
        fq.pct = (fq.raw / bytes.len() as u8).into();
        table.insert(c, fq);
    }
    table
}

/// score a ciphertext's freq table against the ETAOINSHRDLU freq table
fn score_cipher(cipher_table: HashMap<u8, Freq>, key_char: u8) -> FreqScore {
    let mut freq_score = FreqScore { score: 0.0, id: key_char };
    let english_letters = table::char_freq();

    for (c, val) in cipher_table.iter() {
        let mut r = 0.0;
        match english_letters.get(c) {
            // Take the difference of the letter's freq/length of word to the
            // "ideal" frequency in table::freq_table()
            Some(dist) => r += (dist - val.pct).abs(),
            // or add 1 for any keys that aren't in the alphabet;
            // Higher scores indicate lower liklihood of english plaintext.
            None => r += 1.0
        };
        freq_score.score += r;
    }
    freq_score
}
