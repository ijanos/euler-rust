pub fn main() {
    let ispalindrome = |s: &str| s.chars().zip(s.chars().rev()).all(|(x, y)| x == y);
    let ans = (1..1_000_000)
                  .filter(|&n| ispalindrome(&n.to_string()) && ispalindrome(&format!("{:b}", n)))
                  .fold(0, |acc, n| acc + n);
    println!("{}", ans);
}
