use common::digit_fact_sum;

pub fn main() {
    let ans = (3..1_000_000)
                  .filter(|&i| i == digit_fact_sum(i))
                  .fold(0, |sum, i| sum + i);
    println!("{}", ans);
}
