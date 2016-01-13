// p12: highly divisible triangular number

extern crate prime;
extern crate itertools;

use prime::factors;
use itertools::Itertools; // for group_by

pub fn main() {
    let mut triangle = 0;
    for i in 1.. {
        triangle += i;

        let prime_factors = factors(triangle);
        let num_of_factors = prime_factors
            .iter()
            .group_by(|elt| *elt)  // will get things like [(0, vec![0,0,0]), (1, vec![1,1]))]
            .map(|(_, x)| x.len() + 1)
            .fold(1, |x,y| x*y);

        if num_of_factors > 500{
            println!("ret = {}", triangle);
            break;
        }
    }
}
