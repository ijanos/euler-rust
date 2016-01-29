// I thought I need to memoize this but it is plenty fast already
fn collatz_length(a: u32) -> u32 {
    let mut length = 1;
    let mut n: u64 = a as u64;
    while n != 1 {
        if n % 2 == 0 {
            n  /= 2;
        } else {
            n = 3 * n + 1;
        }
        length += 1;
    }
    length
}

pub fn main() {
    let mut max = 0;
    let mut maxi = 0;
    for i in 2..1_000_000 {
        let cl = collatz_length(i);
        if cl > max {
            max = cl;
            maxi = i;
        }
    }
    println!("{}", maxi);
}
