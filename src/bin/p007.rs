// p3.rs
extern crate prime;
use prime::Prime;

pub fn main() {
    let mut prime = Prime::new();

    println!("ret = {}", prime.nth(10000).unwrap());
}
