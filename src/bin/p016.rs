// p16
//
// I cannot think out any solutions besides actually
// compute the value.
//
// Note that 2^1000 is quite large, so no primary type
// is sufficient.
//
// Somehow I don't like to use the `num` crate for bigint.
// so I simulate the multiplication using vectors.



// num is in reversed order of digits.
// for example: 123 is represented as vec![3,2,1].
fn mul_2(num: &mut Vec<u32>) {
    let mut carry = 0;
    for cell in num.iter_mut() {
        carry = 2* (*cell) + carry;

        *cell = carry%10;
        carry /= 10;
    }

    if carry > 0 {
        num.push(carry);
    }
}

pub fn main() {
    let mut num = vec![1];

    for _ in 0..1000 {
        mul_2(&mut num);
    }

    let ret: u32 = num.iter().fold(0, |x,&y| x+y);

    println!("ret = {}", ret);
}
