use permutohedron::LexicalPermutation as Permutations;
use std::char;

pub fn main() {
    let mut input = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    for _ in 1..1_000_000 {
        input.next_permutation();
    }
    let ans = input.iter()
                   .map(|&d| char::from_digit(d, 10).unwrap())
                   .collect::<String>();
    println!("{}", ans);
}
