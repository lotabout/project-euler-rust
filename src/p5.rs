// p5.rs

fn gcd(a : u32, b : u32) -> u32 {
    let mut m = if a > b { a } else { b };
    let mut n = if a > b { b } else { a };

    while n != 0 && m % n != 0 {
        let tmp = n;
        n = m % n;
        m = tmp;
    }

    n
}

pub fn main() {
    let mut ret = 1;

    for x in 1..20 {
        let greatest_common_divisor = gcd(ret, x);
        ret = ret * x / greatest_common_divisor;
    }

    println!("ret = {}", ret);
}

#[cfg(test)]
mod test {
    use super::gcd;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(16, 24), 8);
        assert_eq!(gcd(1, 1), 1);
        assert_eq!(gcd(1, 24), 1);
        assert_eq!(gcd(2, 24), 2);
        assert_eq!(gcd(3, 24), 3);
        assert_eq!(gcd(3, 2), 1);
        assert_eq!(gcd(12, 18), 6);
        assert_eq!(gcd(5, 2), 1);
    }
}
