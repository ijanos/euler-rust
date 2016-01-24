pub fn main() {
    let a = (1..101).map(|x| x * x ).fold(0, |acc, x| acc + x);
    let b: u32 = (1..101).fold(0, |acc, x| acc + x);
    println!("{}", b.pow(2) - a);
}