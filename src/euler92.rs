fn chain89(start: u32) -> bool {
    let mut digits: Vec<u32> = start.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut sum;
    loop {
        sum = digits.iter().fold(0, |acc, n| acc + n*n);
        if sum == 1 { return false; }
        if sum == 89 { return true; }
        digits = sum.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect();
    }
}


pub fn main() {
    println!("{}", (2..10_000_000).filter(|&n| chain89(n)).count());
}