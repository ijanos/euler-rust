use num::bigint::BigUint;
use num::bigint::ToBigUint;

fn factor(n: usize) -> BigUint {
    let mut f = 1.to_biguint().unwrap();
    for i in 1..n + 1 {
        f = f * i.to_biguint().unwrap();
    }
    f
}

pub fn main() {
    let million = 1_000_000.to_biguint().unwrap();
    let choose = |n, r| factor(n) / (factor(r) * factor(n - r));

    let mut ans = 0;
    for n in 1..101 {
        for r in 1..n {
            if choose(n, r) > million {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
