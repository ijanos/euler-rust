use std::collections::{HashMap, HashSet};

pub fn main() {
    let triplet = |n, m, k| {
        let a = k * (m * m - n * n);
        let b = k * (2 * m * n);
        let c = k * (m * m + n * n);
        let mut v = vec![a, b, c];
        v.sort();
        (v[0], v[1], v[2])
    };

    let mut perimeters: HashMap<u32, u32> = HashMap::new();
    let mut seen: HashSet<(u32, u32, u32)> = HashSet::new();

    for n in 1..10u32 {
        for m in n + 1..10 {
            for k in 1..40 {
                let (a, b, c) = triplet(n, m, k);
                if !seen.contains(&(a, b, c)) {
                    *perimeters.entry(a + b + c).or_insert(0) += 1;
                    seen.insert((a, b, c));
                }
            }
        }
    }
    let max = perimeters.iter().filter(|&(&k, _)| k < 1000).max_by_key(|&(_, v)| v).unwrap();
    let ans = perimeters.iter()
                        .filter(|&(&k, &v)| k < 1000 && v == *max.1)
                        .map(|(&k, _)| k)
                        .max()
                        .unwrap();
    println!("{}", ans);
}
