fn digit_fifth_power_sum(n: usize) -> usize {
    n.to_string()
     .chars()
     .map(|c| {
         match c {
             '0' => 0,
             '1' => 1,
             '2' => 32,
             '3' => 243,
             '4' => 1024,
             '5' => 3125,
             '6' => 7776,
             '7' => 16807,
             '8' => 32768,
             '9' => 59049,
             _ => panic!(),
         }
     })
     .fold(0, |sum, i| sum + i)
}

pub fn main() {
    let ans = (2..1_000_000)
                  .filter(|&i| i == digit_fifth_power_sum(i))
                  .fold(0, |sum, i| sum + i);
    println!("{}", ans);
}
