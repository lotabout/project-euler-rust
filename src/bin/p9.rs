pub fn main() {
    for a in 1..333 {
        for b in a..(500 - a/2) {
            let c = 1000 - a - b;
            if a*a + b*b == c*c {
                println!("(a, b, c) = ({}, {}, {}), a*b*c={}", a,b,c,a*b*c);
                return
            }
        }
    }
}
