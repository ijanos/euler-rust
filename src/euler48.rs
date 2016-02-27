use num::bigint::ToBigInt;
use num;

pub fn main() {
    let ans: num::BigInt = (1..1001)
                               .map(|x| num::pow(x.to_bigint().unwrap(), x))
                               .fold(num::zero(), |acc, x| acc + x);
    println!("{}",
             ans.to_string()
                .chars()
                .rev()
                .take(10)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<String>());
}
