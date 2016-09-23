use std::collections::HashMap;
use itertools::Itertools;


fn is_seq(triplet: &Vec<&u32>) -> bool {
    triplet[1] - triplet[0] == triplet[2] - triplet[1]
}

pub fn main() {
    let primes_from1000 = (1000..)
                              .filter(|&n| (2..(n as f32).sqrt() as u32 + 1).all(|i| n % i != 0));
    let mut permutations = HashMap::<String, Vec<u32>>::new();

    for p in primes_from1000.take_while(|&n| n < 10_000) {
        let mut numbers = p.to_string().chars().collect::<Vec<char>>();
        numbers.sort();
        let key = numbers.into_iter().collect::<String>();
        let mut v = permutations.entry(key).or_insert_with(Vec::new);
        v.push(p)
    }

    let mut res = Vec::new();
    for v in permutations.values().filter(|&v| v.len() > 2) {
        let r = v.iter()
                 .combinations_n(3)
                 .filter(|c| is_seq(c))
                 .flatten()
                 .collect::<Vec<_>>();
        if !r.is_empty() {
            res.push(r);
        }
    }
    println!("{:?}", res);
}
