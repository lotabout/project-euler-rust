// p15
//
// This is actually a math problem of counting all
// combinations of the path
//
// The answer is C40/20 = 40!/(20! * 20!)
//
// but the factorial expression is easy to get overflowed.
//

fn gcd(a: u64, b: u64) -> u64 {
    let mut x = if a > b {a} else {b};
    let mut y = if a > b {b} else {a};

    while y != 0 {
        let tmp = x % y;
        x = y;
        y = tmp;
    }
    x
}

fn divide_common(a: u64, b: u64) -> (u64, u64) {
    let tmp = gcd(a, b);
    (a/tmp, b/tmp)
}

fn combinations(n: u64, m: u64) -> u64{
    let (mut top, mut bottom) = (1, 1);

    for i in 1..(m+1) {
        bottom *= i;
    }

    for i in (n-m+1)..(n+1) {
        top *= i;
        let tmp  = divide_common(top, bottom);
        top = tmp.0;
        bottom = tmp.1;
    }

    return top/bottom;
}

pub fn main() {
    println!("{:?}", combinations(40,20));
}
