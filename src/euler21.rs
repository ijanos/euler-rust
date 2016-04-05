pub fn main() {
    let divisor_sum = |n| (1..(n / 2 + 1)).filter(|&i| n % i == 0).fold(0, |sum, i| sum + i);
    let amicable_pair = |a, b| a != b && divisor_sum(a) == b && divisor_sum(b) == a;
    let ans = (1..10_000)
                  .filter(|&a| amicable_pair(a, divisor_sum(a)))
                  .fold(0, |sum, i| sum + i);
    println!("{}", ans);
}
