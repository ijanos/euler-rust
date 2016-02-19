use std::collections::HashSet;

const MAX: u32 = 10_000;

pub fn main() {
    let primes = (1..).filter(|&n| (2..(n as f32).sqrt() as u32 + 1).all(|i| n % i != 0));
    let isprime = |&n| (2..(n as f32).sqrt() as u32 + 1).all(|i| n % i != 0);
    let mut set = HashSet::<u32>::new();
    let doublesquares = (1..).map(|x| 2*(x*x)).take(1500).collect::<Vec<u32>>();

    for p in primes.take_while(|&v| v < MAX) {
        for &s in doublesquares.iter() {
            set.insert(p + s);
        }
    }
    println!("{}", (1..MAX).filter(|x| x % 2 == 1 && !isprime(x) && !set.contains(&x)).nth(0).unwrap());
}
