pub fn main() {
    let primes = || (2..).filter(|&n| (2..(n as f32).sqrt() as u32 + 1).all(|i| n % i != 0));
    let isprime = |n: &u32| (2..(*n as f32).sqrt() as u32 + 1).all(|i| n % i != 0);

    let mut sum = 0;
    let mut primesums = Vec::new();
    for p in primes() {
        sum += p;
        if sum > 1_000_000 { break };
        primesums.push(sum);
    }
    let mut max = 0;
    for &n in &primesums {
        for &&p in &primesums.iter().take_while(|&&p| p < n).collect::<Vec<_>>() {
            let candidate = n - p;
            if isprime(&candidate) && candidate > max {
                max = candidate;
            }
        }
    }
    println!("{}", max);
}
