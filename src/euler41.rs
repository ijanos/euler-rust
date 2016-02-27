use std::char;

pub fn main() {
    let primes = (2..).filter(|&n| (2..(n as f32).sqrt() as u32 + 1).all(|i| n % i != 0));
    let mut max = 0;
    for p in primes.take_while(|&p| p <= 7654321) {
        let s = p.to_string();
        if (1..s.len() + 1)
               .all(|n| s.chars().any(|p| p == char::from_digit(n as u32, 10).unwrap())) {
            if p > max {
                max = p;
            }
        }
    }
    println!("{}", max);
}
