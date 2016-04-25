pub fn main() {
    let is_pandigital = |n: &str| n.len() == 9 && "123456789".chars().all(|i| n.contains(i));
    let mut ans = 0;
    for i in 1..10_000 {
        for j in 2..7 {
            let mut concat = String::new();
            for n in 1..j {
                concat.push_str(&(i * n).to_string());
            }
            if is_pandigital(&concat) {
                let num: u32 = concat.parse().unwrap();
                if num > ans {
                    ans = num;
                }
            };
        }
    }
    println!("{}", ans);
}
