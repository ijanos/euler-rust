use num::rational::Ratio;

fn remove_same_digit(a: u32, b: u32) -> Option<(u32, u32)> {
    let a1 = a % 10;
    let a2 = (a - a1) / 10;
    let b1 = b % 10;
    let b2 = (b - b1) / 10;
    match (a1, a2, b1, b2) {
        (a1, a2, b1, b2) if a1 == b1 && b2 != 0 => Some((a2, b2)),
        (a1, a2, b1, b2) if a1 == b2 && b1 != 0 => Some((a2, b1)),
        (a1, a2, b1, b2) if a2 == b1 && b2 != 0 => Some((a1, b2)),
        (a1, a2, b1, b2) if a2 == b2 && b1 != 0 => Some((a1, b1)),
        _ => None,
    }
}

pub fn main() {
    let mut ans = Ratio::new(1, 1);
    for i in 11..100 {
        for j in i + 1..100 {
            if let Some((a, b)) = remove_same_digit(i, j) {
                let r1 = Ratio::new(i, j);
                let r2 = Ratio::new(a, b);
                if r1 == r2 && i / a != 10 {
                    ans = ans * r1;
                }
            }
        }
    }
    println!("{}", ans.denom());
}
