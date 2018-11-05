/// #3 given a hex-encoded string H of length L, xor each byte in H with a L-length string of
/// each character in the alphabet
///
use std::collections::HashMap;

fn main() {
    let mut shmap = HashMap::new();

    shmap.insert(0x41, "a");
    println!("HI {:?}", shmap.get(&0x41))
}
