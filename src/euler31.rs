pub fn main() {
    let coin_sum = |a, b, c, d, e, f, g| 1 * a + 2 * b + 5 * c + 10 * d + 20 * e + 50 * f + 100 * g;
    let mut ans = 0;
    for a in 0..201 {
        for b in 0..101 - a / 2 {
            for c in 0..41 - b / 3 {
                for d in 0..21 - c / 2 {
                    for e in 0..11 - d / 2 {
                        for f in 0..5 {
                            for g in 0..3 {
                                if coin_sum(a, b, c, d, e, f, g) == 200 {
                                    ans += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{}", ans + 1); // plus one for single £2 coin
}
