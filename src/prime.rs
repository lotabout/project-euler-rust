// library for dealing with prime numbers.

// 1. currently, no wheel optimization had been added.
// 2. I feel that the code is rather ugly.
// TODO: write more clear code after getting more familiar with rust.

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::iter::Cycle;
use std::slice::Iter;

// store the running state of generating prime
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct RunningState {
    value: i32,
    prime: i32,
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
    position: i32,
}

impl Prime {
    pub fn new() -> Self {
        let mut ret = Prime {
            running: BinaryHeap::new(),
            position: 1,
        };

        ret
    }
}

impl Iterator for Prime {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        // try next position
        self.position += 1;

        // check if current position is already crossed out
        //println!("begin");
        //for i in self.running.iter() {
            //println!("{:?}", i);
        //}
        while let Some(&RunningState{value, ..}) = self.running.peek() {
            // self.position is a valid prime
            if self.position != value {
                break;
            }

            // pop the current crossed value and insert the next one
            while self.position == self.running.peek().unwrap().value {
                if let Some(RunningState{value, prime}) = self.running.pop() {
                    self.running.push(RunningState{
                        value: value + prime,
                        prime: prime,
                    });
                }
            }

            // try next position
            self.position += 1;
        }

        self.running.push(RunningState{
            value: self.position * self.position,
            prime: self.position,
        });

        //println!("After");
        //for i in self.running.iter() {
            //println!("{:?}", i);
        //}

        Some(self.position)
    }
}

mod test {
    use super::Prime;

    #[test]
    fn test_prime() {
        let mut primes = Prime::new();

        assert_eq!(primes.next(), Some(2));
        assert_eq!(primes.next(), Some(3));
        assert_eq!(primes.next(), Some(5));
        assert_eq!(primes.next(), Some(7));
        assert_eq!(primes.next(), Some(11));
        assert_eq!(primes.next(), Some(13));
        assert_eq!(primes.next(), Some(17));
        assert_eq!(primes.next(), Some(19));
        assert_eq!(primes.next(), Some(23));
        assert_eq!(primes.next(), Some(29));
    }
}
