use num::BigUint;
use num::bigint::ToBigUint;

pub struct Fib {
    pub curr: BigUint,
    pub next: BigUint,
}

impl Iterator for Fib {
    type Item = BigUint;
    fn next(&mut self) -> Option<BigUint> {
        let new = &self.curr + &self.next;

        self.curr = self.next.clone();
        self.next = new;

        Some(self.curr.clone())
    }
}


pub fn main() {
    let mut fib = Fib {
        curr: 1.to_biguint().unwrap(),
        next: 1.to_biguint().unwrap(),
    };
    let ans = fib.position(|i| i.to_string().len() == 1_000).unwrap();
    println!("{}", ans + 2); // plus 2 because the iterator starts at F_3
}
