extern crate prime;

pub fn main() {
    let primes = prime::Prime::new();

    let result = primes.take_while(|x| *x < 2000000).fold(0, |x, y| x + y);
    println!("ret = {}", result);
}
