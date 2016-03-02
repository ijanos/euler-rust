fn array2num(numbers: &[char]) -> u32 {
    let mut buf = String::with_capacity(numbers.len());
    for n in numbers.iter() {
        buf.push(*n);
    }
    buf.parse().unwrap()
}

fn truncatable(input: &str) -> bool {
    let input: Vec<char> = input.chars().collect();
    let isprime = |&n| n > 1 && (2..(n as f32).sqrt() as u32 + 1).all(|i| n % i != 0);
    for i in 1..input.len() {
        let t1 = &array2num(&input[0..i]);
        let t2 = &array2num(&input[i..input.len()]);
        if !isprime(t1) || !isprime(t2) {
            return false;
        }
    }
    true
}

pub fn main() {
    let primes = (10 as u64..)
                     .filter(|&n| (2..(n as f32).sqrt() as u32 + 1).all(|i| n % i as u64 != 0));
    let banned = ['0', '4', '6', '8'];
    let ans = primes.filter(|&p| !p.to_string().chars().any(|c| banned.contains(&c)))
                    .filter(|&p| truncatable(&p.to_string()))
                    .take(11)
                    .fold(0, |acc, n| acc + n);
    println!("{}", ans);
}
