
pub fn main() {
    let primes = (2 as u64..)
                     .filter(|&n| (2..(n as f32).sqrt() as u32 + 1).all(|i| n % i as u64 != 0));
    println!("{}",
             primes.take_while(|&n| n < 2_000_000).fold(0, |acc, n| acc + n));
}
