use num;
use num::bigint::ToBigUint;

pub fn main() {
    let two = 2.to_biguint().unwrap();
    println!("{}",
             num::pow(two, 1000)
                 .to_string()
                 .chars()
                 .map(|c| c.to_digit(10).unwrap())
                 .fold(0, |sum, i| sum + i));
}
