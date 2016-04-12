use std::collections::HashSet;
use num;
use num::bigint::ToBigUint;
use num::bigint::BigUint;


pub fn main() {
    let mut set: HashSet<BigUint> = HashSet::new();
    for a in 2..101 {
        let a = a.to_biguint().unwrap();
        for b in 2..101 {
            set.insert(num::pow(a.clone(), b));
        }
    }
    println!("{}", set.len());
}
