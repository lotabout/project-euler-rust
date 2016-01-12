// p4.rs

fn is_palindrom(num: u32) -> bool {
    let num_str = num.to_string();
    let num_str_rev : String = num_str.chars().rev().collect();
    num_str == num_str_rev
}

pub fn main() {
    let mut ret = 0;

    for x in (100..999) {
        for y in (x..999) {
            let mul = x * y;
            if is_palindrom(mul) && mul > ret{
                ret = mul;
            }
        }
    }

    println!("result = {}", ret);
}

#[cfg(test)]
mod test {
    use super::is_palindrom;
    #[test]
    fn test_palindrom() {
        assert_eq!(is_palindrom(2332), true);
        assert_eq!(is_palindrom(22), true);
        assert_eq!(is_palindrom(2), true);
        assert_eq!(is_palindrom(23132), true);
        assert_eq!(is_palindrom(233), false);
    }
}
