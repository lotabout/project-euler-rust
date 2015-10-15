// p3.rs
mod prime;
use prime::factors;

pub fn main() {
    let factor = factors(600851475143);

    let ret = factor.last().unwrap();

    println!("ret = {}", ret);
}
