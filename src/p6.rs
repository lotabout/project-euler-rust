// p6.rs

fn p6(num : i64) -> i64 {
    let sum_of_sq = ((1..(num+1)).fold(0, |acc, x| acc+x) as i64).pow(2);
    let sq_of_sum = (1..(num+1)).map(|x: i64| x.pow(2)).fold(0, |acc,x| acc+x);

    sum_of_sq - sq_of_sum
}

#[test]
fn test_p6() {
    assert_eq!(p6(10), 2640);
}

pub fn main() {

    let ret = p6(100);

    println!("ret = {}", ret);
}
