pub fn main() {
    let mut primes = (2..).filter(|&n| (2..(n as f32).sqrt() as u32 + 1).all(|i| n % i != 0));
    println!("{}", primes.nth(10_000).unwrap());
}
