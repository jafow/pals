use std::error;
use std::fmt;

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
fn find_key_size(ciphertext: &str) -> Result<Vec<u8>, PalsError> {
    let mut keysizes: Vec<u8> = vec![];
    
    let bytes = ciphertext.as_bytes();
    for size in 2..6 {
        let mut iter = bytes.chunks(size as usize);
        let a = iter.next().unwrap();
        dbg!(&a);
        let b = iter.next().unwrap();
        dbg!(&b);
        let hamm: u8 = xor::hamming_distance_from_slice(a, b) / &size;
        // dbg!(&hamm);
        keysizes.push(hamm);
        // dbg!(&hamming_left);

        // let c = iter.next().expect("get chunk"); 
        // let d = iter.next().expect("get chunk");  
        // let hamming_right = xor::hamming_distance_from_slice(c, d);
        // dbg!(&hamming_right);
    }
    Ok(keysizes)
}

#[test]
fn find_key_size_test() {
    assert_eq!(find_key_size("1a0d131c44191507150d0e411504090f125e4513030f5216101f111b01441a131b1f01194f"), Ok(vec![5]));
}
