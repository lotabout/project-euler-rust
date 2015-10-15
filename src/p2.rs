// fibonacci

struct Fibo {
    curr: i32,
    next: i32,
}

impl Fibo {
    pub fn new() -> Fibo {
        Fibo {
            curr: 1,
            next: 1,
        }
    }
}

impl Iterator for Fibo {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

mod test {
    #[test]
    fn fibo() {
        let mut fibo_seq = Fibo::new();

        assert_eq!(fibo_seq.next(), Some(1));
        assert_eq!(fibo_seq.next(), Some(2));
        assert_eq!(fibo_seq.next(), Some(3));
        assert_eq!(fibo_seq.next(), Some(5));
        assert_eq!(fibo_seq.next(), Some(8));
        assert_eq!(fibo_seq.next(), Some(13));
        assert_eq!(fibo_seq.next(), Some(21));
    }
}

pub fn main() {
    let mut result = 0;
    let fibo_seq = Fibo::new();

    for x in fibo_seq.take_while(|x| *x < 4000000).filter(|x| (*x) % 2 == 0) {
        result += x;
    }

    println!("result = {}", result);

}
