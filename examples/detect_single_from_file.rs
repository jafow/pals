/// read a file of 60 strings and find the one that has been 
/// XORd with a single byte
use std::fs::File;
use std::io::prelude::*;

extern crate pals;

fn main() {
    let mut f = File::open("data/4.txt").expect("Should open a file");

    let mut lines = String::new();
    f.read_to_string(&mut lines)
        .expect("error reading contents of file");

    for l in lines.lines() {
    }

    assert_eq!(0, 0);
}
