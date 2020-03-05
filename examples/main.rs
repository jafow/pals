extern crate hex;
extern crate pals;

fn main() {
    // println!("huh?");
    // let x = pals::xor::single_byte("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    // println!("{:?}", x.score);
    // dbg!(x);
    let d = pals::xor::hamming_distance("a", "b");
    dbg!(&d);
}
