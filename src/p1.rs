// p1.rs

struct Multiple {
    position: i32,
}

impl Multiple {
    pub fn new() -> Self {
        Multiple {
            position: 0,
        }
    }
}

impl Iterator for Multiple {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = self.position;
        for i in ((self.position + 1) ..) {
            if (i % 3 == 0) || (i % 5 == 0) {
                result = i;
                break;
            }
        }

        self.position = result;
        Some(result)
    }
}

pub fn main() {
    let multiple = Multiple::new();

    let mut result = 0;
    for x in multiple.take_while(|x| *x < 1000) {
        result += x;
    }

    println!("{:?}", result);
}
