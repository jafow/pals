pub mod xor;
pub use xor::{xor_fixed, single_byte};

pub struct FreqScore {
    pub id: u8,
    pub score: f32
}
pub mod table;
pub use table::{char_freq};
