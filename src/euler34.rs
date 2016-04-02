pub fn digit_fact_sum(n: usize) -> usize {
    n.to_string()
     .chars()
     .map(|c| {
         match c {
             '0' => 1,
             '1' => 1,
             '2' => 2,
             '3' => 6,
             '4' => 24,
             '5' => 120,
             '6' => 720,
             '7' => 5040,
             '8' => 40320,
             '9' => 362880,
             _ => panic!(),
         }
     })
     .fold(0, |sum, i| sum + i)
}

pub fn main() {
    let ans = (3..1_000_000)
                  .filter(|&i| i == digit_fact_sum(i))
                  .fold(0, |sum, i| sum + i);
    println!("{}", ans);
}
