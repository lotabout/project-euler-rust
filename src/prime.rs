// library for dealing with prime numbers.

// 1. currently, no wheel optimization had been added.
// 2. I feel that the code is rather ugly.
// TODO: write more clear code after getting more familiar with rust.

use std::cmp::Ordering;
use std::collections::BinaryHeap;

// Wheel will be an iterator, given small primes
//
// # Example
// let mut wheel = Wheel::new([2,3].to_vec());
// println!('{}', wheel.next().wrap());
struct Wheel {
    initials: std::vec::IntoIter<u64>,

    // for cycling increment
    steps: Vec<u64>,
    current_step: usize,

    // current position
    next: u64,

    // running status
    state: WheelState,
}

enum WheelState {
    Front,
    Back,
}

impl Wheel {
    pub fn new(v: Vec<u64>) -> Self {
        let product = v.iter().fold(1, |x, y| x * y);
        let mut initials = v.to_vec();
        let mut spokes = Vec::new();

        for i in *v.last().unwrap()..(product+1) {
            if v.iter().filter(|&x| i % x == 0).count() <= 0 {
                // valide spoke
                initials.push(i);
                spokes.push(i);
            }
        }
        spokes.push(product+1);

        let mut wheel_steps = Vec::new();
        let mut last = 1;
        for i in spokes {
            wheel_steps.push(i - last);
            last = i;
        }

        Wheel {
            initials: initials.into_iter(),
            steps: wheel_steps,
            current_step: 0,
            next: product+1,
            state: WheelState::Front,
        }
    }
}

impl Iterator for Wheel {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        match self.state {
            WheelState::Front => match self.initials.next() {
                elt @ Some(..) => elt,
                None => {
                    self.state = WheelState::Back;
                    Some(self.next)
                }
            },
            WheelState::Back => {
                self.next += self.steps[self.current_step];
                self.current_step = (self.current_step + 1)%self.steps.len();
                Some(self.next)
            },
        }
    }
}


// store the running state of generating prime
#[derive(Copy, Clone, Eq, PartialEq, Debug)]

// a state will contain (prime, factor) which indicate that (prime * factor) will be crossed out
struct RunningState {
    value: u64,
    prime: u64,
}

impl Ord for RunningState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.value.cmp(&self.value)
    }
}

impl PartialOrd for RunningState{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// The prime number generator
pub struct Prime {
    running: BinaryHeap<RunningState>,
    wheel: Wheel,
}

const SMALL_PRIMES: &'static [u64] = &[2,3,5,7];

impl Prime {
    pub fn new() -> Self {
        Prime {
            running: BinaryHeap::new(),
            wheel: Wheel::new(SMALL_PRIMES.to_vec()),
        }
    }
}

impl Iterator for Prime {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_try = self.wheel.next().unwrap();

        while let Some(&RunningState{prime, value}) = self.running.peek() {
            // next_try is a prime
            if next_try < value {
                break;
            } else if next_try == value {
                // try next value
                next_try = self.wheel.next().unwrap();
            }

            // pop the current crossed value and insert the next one
            let mut next_value = value + prime;
            while next_try > next_value{
                next_value += prime;
            }

            self.running.pop();
            self.running.push(RunningState{
                prime: prime,
                value: next_value,
            });
        }

        // skip the initial primes
        if next_try > 7 {
            self.running.push(RunningState{
                value: next_try * next_try,
                prime: next_try,
            });
        }

        Some(next_try)
    }
}

pub fn factors(orig: u64) -> Vec<u64> {
    let mut primes = Prime::new();
    let mut ret = Vec::new();

    let mut running_val = orig;
    while running_val > 1 {
        let current_prime = primes.next().unwrap();
        //println!("current_prime = {}", current_prime);

        while running_val % current_prime == 0 {
            ret.push(current_prime);

            running_val /= current_prime;
            //println!("inner: running_val = {}", running_val);
        }
    }

    ret
}

#[allow(dead_code)]
#[cfg(test)]
mod test {
    use super::Prime;
    use super::Wheel;

    const WHEEL_TEST: &'static [u64] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 25, 29, 31, 35, 37, 41, 43, 47, 49, 53];

    #[test]
    fn test_wheel() {
        let mut wheel = Wheel::new([2,3].to_vec());

        for p in WHEEL_TEST {
            let current = wheel.next().unwrap();
            assert_eq!(current, *p);
        }
    }


    const SMALL_PRIMES_TEST: &'static [u64] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53,
                                                59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109,
                                                113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
                                                181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241,
                                                251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313,
                                                317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389,
                                                397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461,
                                                463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547,
                                                557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617,
                                                619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691,
                                                701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773,
                                                787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859,
                                                863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947,
                                                953, 967, 971, 977, 983, 991, 997];

    #[test]
    fn test_prime() {
        let mut primes = Prime::new();

        for p in SMALL_PRIMES_TEST {
            let current = primes.next().unwrap();
            assert_eq!(current, *p);
        }
    }

    #[test]
    fn test_factors() {
        use super::factors;

        assert_eq!(factors(1), vec![]);
        assert_eq!(factors(13), vec![13]);
        assert_eq!(factors(24), vec![2,2,2,3]);
        assert_eq!(factors(25), vec![5,5]);
        assert_eq!(factors(99), vec![3,3,11]);
    }
}
