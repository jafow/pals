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

// pub fn hamming_distance() {
    
// }
