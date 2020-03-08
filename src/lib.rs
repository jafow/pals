extern crate base64;
use std::error;
use std::fmt;
// use std::fs::{self, File};

// use base64;

pub mod xor;
pub use xor::{single_byte, xor_fixed};

#[derive(Debug, Clone)]
pub struct FreqScore {
    pub id: u8,
    pub score: i32,
}

#[derive(Clone)]
pub struct LineScore {
    pub fscore: FreqScore,
    pub line: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct KeySizeGuess {
    keysize: u8,
    hamming: u8
}

pub mod table;
pub use table::char_freq;

#[derive(Debug, PartialEq)]
pub enum PalsError {
    Keysize(String),
    Other(String),
}

impl fmt::Display for PalsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PalsError::Keysize(ref e) => e.fmt(f),
            PalsError::Other(ref s) => f.write_str(&**s),
        }
    }
}

impl error::Error for PalsError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<std::boxed::Box<dyn error::Error + std::marker::Send + std::marker::Sync>> for PalsError {
    fn from(x: Box<dyn error::Error + std::marker::Send + std::marker::Sync>) -> PalsError {
        PalsError::Other(x.to_string())
    }
}

// pub fn hamming_distance() {
    
// }
//
//

// chunk pairs
// given a slice, fold it down to a list of chunksize pairs arrays
fn chunk_pairs(ciphertext: &[u8], chunksize: u8) -> Vec<Vec<u8>> {
    let mut iter = ciphertext.chunks(chunksize as usize);
    let mut res: Vec<u8> = Vec::new();

    // while let next = Some(iter.next()) {
    //     let rhs = 

    // }

    // iter.fold(res, |acc, &current| {
    //     if acc[acc.len()].len() < 2 {
    //         acc[acc.len()].push(&current);
    //     } else {
    //         acc.push(vec![&current]);
    //     }
    // });
    vec![res]
}

#[test]
fn foo_inner() {
    
}

pub fn find_key_size(ciphertext: &[u8]) -> Result<Vec<u8>, PalsError> {
    let mut keysizes: Vec<u8> = vec![];
    
    for size in 2..(ciphertext.len() / 2) {
        // chunk into slices 2x size and then split them in half before passing to hamming distance
        let mut distances: Vec<u8> = Vec::new();
        for chunk in ciphertext.chunks(size * 2 as usize) {
            if chunk.len() < size {
                break
            }
            let edit_distance = xor::hamming_distance_from_slice(&chunk[0..size], &chunk[size..]);
            match edit_distance {
                Some(distance) => distances.push(distance / size as u8),
                None => break
            }
        }
        let avg: u8 = distances.iter().sum::<u8>() / distances.len() as u8;
        keysizes.push(size as u8);
    }
    keysizes.sort();
    Ok(keysizes)
}

#[test]
fn find_key_size_test() {
    assert_eq!(find_key_size(b"1a0d131c44191507150d0e411504090f125e4513030f5216101f111b01441a131b1f01194f"), Ok(vec![KeySizeGuess{ hamming: 2, keysize: 5 }]));
    // assert_eq!(find_key_size(b"1a0d131c44191507150d0e411504090f125"), Ok(vec![KeySizeGuess{ hamming: 2, keysize: 5 }]));
    // assert_eq!(
    //     find_key_size(b"0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f")
    //     , Ok(vec![3, 6, 7])
    // );

    // let ciphertext_file = fs::read_to_string("data/6a.txt").expect("read file");
    // let b64_decoded = base64::decode(&ciphertext_file).expect("decoded file");
    // println!("b64====== {:?}", b64_decoded);
    // assert_eq!(Ok(vec![5, 29]), find_key_size(&b64_decoded[..]));
    // assert_eq!(
    //     find_key_size(
    //         b"236f222f272131222f206924622427203a282721626e3d2c2c242b3c692a306f2f6e3d242b23213c656526202d3a263762203c6e28652e2e39372c37622e6e3d262926262b3c692a306f2f6e3a242b23213c")
    //     , Ok(vec![6])
    // );
}
