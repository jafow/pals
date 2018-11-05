use std::collections::HashMap;

/// character frequency of English language
/// source: https://en.wikipedia.org/wiki/Letter_frequency
pub fn char_freq<'a, T>() -> HashMap<u32, f32> {
    let freq_table: HashMap<u32, f32> = vec![
        (0x41, 0.08167),
        (0x42, 0.01492),
        (0x43, 0.02782),
        (0x44, 0.04253),
        (0x45, 0.12702),
        (0x46, 0.02228),
        (0x47, 0.02015),
        (0x48, 0.06094),
        (0x49, 0.06966),
        (0x4a, 0.00153),
        (0x4b, 0.00772),
        (0x4c, 0.04025),
        (0x4d, 0.02406),
        (0x4e, 0.06749),
        (0x4f, 0.07507),
        (0x50, 0.01929),
        (0x51, 0.00095),
        (0x52, 0.05987),
        (0x53, 0.06327),
        (0x54, 0.09056),
        (0x55, 0.02758),
        (0x56, 0.00978),
        (0x57, 0.02360),
        (0x58, 0.00150),
        (0x59, 0.01974),
        (0x5a, 0.00074)
    ].iter().cloned().collect();
    freq_table
}

pub const LETTERS: [u8; 2] = [b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h',
    b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's',
    b't', b'u', b'v', b'w', b'x', b'y', b'z'];
