use std::fs::{self, File};
use std::str;

extern crate hex;
extern crate pals;


pub fn main() -> std::io::Result<()> {
    // secret
    let secret_key = "PIZZA DAD";

    // read the hexencoded plaintext
    let ciphertext: String = fs::read_to_string("data/freediehexnonewlin.txt").expect("read file");
    let hexd = hex::decode(&ciphertext).unwrap();
    let hexds = str::from_utf8(&hexd).unwrap();
    // encrypt with repeating key xor
    let roo = pals::xor::xor_repeating_key(&hexds, secret_key).expect("working");

    // save to a file
    let hexe = hex::encode(str::from_utf8(&roo).unwrap());
    fs::write("data/freedie-cipher.txt", hexe)?;

    // read it back out
    let ciphertext = fs::read_to_string("data/freedie-cipher.txt")?;
    let decoded = hex::decode(ciphertext).expect("working");
    let roo = pals::xor::xor_repeating_key(str::from_utf8(&decoded).unwrap(), secret_key).expect("working");

    // say hello freddie
    println!("{:?}", str::from_utf8(&roo).unwrap());

    // find the keysize
    let ciphertext = fs::read_to_string("data/freedie-cipher.txt")?;
    println!("ciphertext len is {:?}", &ciphertext.len());

    let decoded = hex::decode(ciphertext).expect("working");
    let keyguess = pals::find_key_size_by_mode(&decoded);

    assert_eq!(Ok(vec![secret_key.len() as u8]), keyguess);
    println!("secret len is {:?}", secret_key.len());
    println!("key guess {:?}", keyguess);
    Ok(())
}

