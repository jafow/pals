/// #3 given a hex-encoded string H of length L, xor each byte in H with a L-length string of
/// each character in the alphabet
///
extern crate pals;
extern crate hex;

fn main() {
    let ciphertext = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let dcipher = hex::decode(ciphertext).expect("hex decoded");

    let key = pals::xor::single_byte(ciphertext);
    // let key: u8 = 88;

    let decrypted = dcipher.iter().map(|x| x ^ key).collect();
    println!("Dec: {:?}", String::from_utf8(decrypted));
}
