pub fn main() {
    let sequence = (0..).flat_map(|i| i.to_string().chars().collect::<Vec<_>>());
    let indicies = [1, 10, 100, 1_000, 10_000, 100_000, 1_000_000];
    let ans = sequence.enumerate()
                      .take(1_000_000)
                      .filter(|&(i, _)| indicies.contains(&i))
                      .fold(1, |acc, (_, n)| n.to_digit(10).unwrap() * acc);
    println!("{}", ans);
}
