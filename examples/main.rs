extern crate pals;
extern crate hex;

fn main() {
    println!("huh?");
    pals::xor::single_byte("48656c6c6f20776f726c6421").expect("should score");
}
