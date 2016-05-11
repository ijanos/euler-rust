use num;
use num::bigint::ToBigUint;

pub fn main() {
    let mut max = 0;
    for i in 1..100 {
        for j in 1..100 {
            let a = i.to_biguint().unwrap();
            let digitsum = num::pow(a, j)
                               .to_string()
                               .chars()
                               .map(|c| c.to_digit(10).unwrap())
                               .fold(0, |sum, x| sum + x);
            if digitsum > max {
                max = digitsum
            }
        }
    }
    println!("{:?}", max);
}
