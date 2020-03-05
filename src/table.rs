use std::collections::HashMap;

/// character frequency of English language
/// source: https://en.wikipedia.org/wiki/Letter_frequency
pub fn char_freq() -> HashMap<u8, i32> {
    let freq_table: HashMap<u8, i32> = vec![
        (0x41, 8167),
        (0x42, 1492),
        (0x43, 2782),
        (0x44, 4253),
        (0x45, 12702),
        (0x46, 2228),
        (0x47, 2015),
        (0x48, 6094),
        (0x49, 6966),
        (0x4a, 0153),
        (0x4b, 0772),
        (0x4c, 4025),
        (0x4d, 2406),
        (0x4e, 6749),
        (0x4f, 7507),
        (0x50, 1929),
        (0x51, 95),
        (0x52, 5987),
        (0x53, 6327),
        (0x54, 9056),
        (0x55, 2758),
        (0x56, 0978),
        (0x57, 2360),
        (0x58, 0150),
        (0x59, 1974),
        (0x5a, 74),
        (0x61, 8167),
        (0x62, 1492),
        (0x63, 2782),
        (0x64, 4253),
        (0x65, 12702),
        (0x66, 2228),
        (0x67, 2015),
        (0x68, 6094),
        (0x69, 6966),
        (0x6a, 0153),
        (0x6b, 0772),
        (0x6c, 4025),
        (0x6d, 2406),
        (0x6e, 6749),
        (0x6f, 7507),
        (0x70, 1929),
        (0x71, 95),
        (0x72, 5987),
        (0x73, 6327),
        (0x74, 9056),
        (0x75, 2758),
        (0x76, 0978),
        (0x77, 2360),
        (0x78, 0150),
        (0x79, 1974),
        (0x7a, 74),
    ].iter()
    .cloned()
    .collect();
    freq_table
}

pub const LETTERS: [u8; 52] = [
    b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p',
    b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'A', b'B', b'C', b'D', b'E', b'F',
    b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V',
    b'W', b'X', b'Y', b'Z',
];
