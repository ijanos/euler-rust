use std::collections::HashSet;

pub fn main() {
    let is_pandigital = |a: u32, b: u32, c: u32| {
        let nums = a.to_string() + &b.to_string() + &c.to_string();
        if nums.len() != 9 || nums.contains('0') {
            false
        } else {
            nums.chars()
                .collect::<HashSet<char>>()
                .len() == 9
        }
    };
    let mut products = HashSet::<u32>::new();
    for a in 1..9876 {
        for b in a..9876 {
            if is_pandigital(a, b, a * b) {
                products.insert(a * b);
            }
        }
    }
    println!("{}", products.iter().fold(0, |sum, x| sum + x));
}
