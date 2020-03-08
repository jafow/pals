extern crate base64;
use std::error;
use std::f64;
use std::fmt;
use std::fs::{self, File};

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
    hamming: u8,
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

fn normalized_edit_distance(ciphertext: &[u8], size: usize) -> f64 {
    // chunk into slices 2x size and then split them in half before passing to hamming distance
    let mut distances: Vec<f64> = Vec::new();
    // let size = *_size as usize;
    for chunk in ciphertext.chunks(size * 2) {
        if chunk.len() < size {
            break
        }
        let edit_distance = xor::hamming_distance_from_slice(&chunk[0..size], &chunk[size..]);
        match edit_distance {
            Some(distance) => distances.push(distance as f64),
            None => break
        }
    }
    let normalized_averaged_edit_distance: f64 = distances.iter().sum::<f64>() / distances.len() as f64 / size as f64;
    normalized_averaged_edit_distance
}

pub fn find_key_size(ciphertext: &[u8]) -> Result<Vec<u8>, PalsError> {
    let ranges: (usize, usize) = (2, 41);
    let mut best_guess: (usize, f64) = (0, f64::MAX);

    let size_range = ranges.0..ranges.1;
    let size_iter = size_range.map(|size| (size, normalized_edit_distance(ciphertext, size)));
    for d in size_iter {
        let (_, edit_distance) = d;
        if edit_distance < best_guess.1 {
            best_guess = d;
        }
    }
    Ok(vec![best_guess.0 as u8])
}

#[test]
fn find_key_size_test() {
    let ciphertext_file = fs::read_to_string("data/6a.txt").expect("read file");
    let b64_decoded = base64::decode(&ciphertext_file).expect("decoded file");
    assert_eq!(Ok(vec![29]), find_key_size(&b64_decoded[..]));
}
