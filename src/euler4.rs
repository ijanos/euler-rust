// fn ispalindrome(n: u32) -> bool {
//    let s = n.to_string();
//    s == s.chars().rev().collect::<String>()
// }

fn ispalindrome(n: u32) -> bool {
    let s = n.to_string();
    s.chars().zip(s.chars().rev()).all(|(x, y)| x == y)
}

pub fn main() {
    let mut max = 0;
    for i in 100..1000 {
        for j in 100..1000 {
            let n = i * j;
            if n > max && ispalindrome(n) {
                max = n;
            }
        }
    }
    println!("{}", max);
}
