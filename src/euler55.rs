use num::BigUint;
use num::bigint::ToBigUint;

pub fn main() {
    let ans = (1..10_000).filter(|n| is_lychrel(n)).count();
    println!("{}", ans);
}

fn is_lychrel(n: &u64) -> bool {
    let is_palindrom = |l: &str| l.chars().zip(l.chars().rev()).all(|(i, j)| i == j);
    let reverse_add = |n: &BigUint| {
        n + n.to_string().chars().rev().collect::<String>().parse::<BigUint>().unwrap()
    };

    let mut n = n.to_biguint().unwrap();
    for _ in 0..50 {
        let m = reverse_add(&n);
        if is_palindrom(&m.to_string()) {
            return false;
        }
        n = m;
    }
    true
}
