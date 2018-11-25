pub mod xor;
pub use xor::{xor_fixed, single_byte};

#[derive(Clone)]
pub struct FreqScore {
    pub id: u8,
    pub score: f32
}

#[derive(Clone)]
pub struct LineScore {
    pub fscore: FreqScore,
    pub line: String
}

pub mod table;
pub use table::{char_freq};
