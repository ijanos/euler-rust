use std::char;

pub fn main() {
    let primes = (2..).filter(|&n| (2..(n as f32).sqrt() as u32 + 1).all(|i| n % i != 0));
    let pandigital = |number: &u32| {
        let s = number.to_string();
        (1..s.len() + 1).all(|n| s.chars().any(|p| p == char::from_digit(n as u32, 10).unwrap()))
    };
    let ans = primes.take_while(|&p| p <= 7654321).filter(pandigital).last().unwrap();
    println!("{}", ans);
}
