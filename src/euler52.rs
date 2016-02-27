use std::collections::HashSet;

fn same_digits(a: u64, b: u64) -> bool {
    let astr = a.to_string();
    let bstr = b.to_string();

    if astr.len() != bstr.len() {
        return false;
    }

    let aset: HashSet<char> = astr.chars().collect();
    let bset: HashSet<char> = bstr.chars().collect();

    aset == bset
}

pub fn main() {
    println!("{}",
             (1..)
                 .filter(|&n| {
                     [6 * n, 5 * n, 4 * n, 3 * n, 2 * n].iter().all(|&m| same_digits(n, m))
                 })
                 .nth(0)
                 .unwrap());
}
