use std::collections::HashSet;

pub fn main() {
    let primes = (2 as i32..)
                     .filter(|&n| (2..(n as f32).sqrt() as u32 + 1).all(|i| n % i as i32 != 0));
    let primes = primes.take(1_000).collect::<HashSet<_>>();
    let formula = |a, b| move |n| n * n + a * n + b;


    let mut max = 0;
    let mut ans = 0;
    for a in -1000..1001 {
        for b in -1001..1001 {
            let form = formula(a, b);
            let prime_length = (0..).map(form).take_while(|x| primes.contains(x)).count();
            if max < prime_length {
                max = prime_length;
                ans = a * b
            }
        }
    }
    println!("{}", ans);
}
