use num::bigint::ToBigInt;

pub fn main() {
    let mut f = 1.to_bigint().unwrap();

    for n in 1..101 {
        f = n.to_bigint().unwrap() * &f;
    }
    let sum = f.to_string().chars().fold(0, |acc, n| acc + n.to_digit(10).unwrap());

    println!("{}", sum);
}
