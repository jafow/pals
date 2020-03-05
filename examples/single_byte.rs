extern crate hex;
/// #3 given a hex-encoded string H of length L, xor each byte in H with a L-length string of
/// each character in the alphabet
///
extern crate pals;

fn main() {
    let ciphertext = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let dcipher = hex::decode(ciphertext).expect("hex decoded");

    let key = pals::xor::single_byte(ciphertext);

    let decrypted = dcipher.iter().map(|x| x ^ key.id).collect();
    println!("Dec: {:?}", String::from_utf8(decrypted));
}
