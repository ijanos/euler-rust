use std::collections::HashMap;

pub fn main() {
    let mut triangle = (1..).map(|n| n*(n+1)/2);
    let mut pentagonal = (1..).map(|n| n*(3*n-1)/2);
    let mut hexagonal = (1..).map(|n| n*(2*n-1));
    let mut current = HashMap::<char, u64>::new();

    current.insert('t', triangle.next().unwrap());
    current.insert('p', pentagonal.next().unwrap());
    current.insert('h', hexagonal.next().unwrap());

    loop {
        let (&k, _) = current.iter().min_by_key(|&(_, v)| v).unwrap();
        let v = match k {
            't' => triangle.next().unwrap(),
            'p' => pentagonal.next().unwrap(),
            'h' => hexagonal.next().unwrap(),
            _ => panic!()
        };
        current.insert(k, v);
        if current.get(&'t').unwrap() == current.get(&'p').unwrap() &&
           current.get(&'p').unwrap() == current.get(&'h').unwrap() &&
           *current.get(&'t').unwrap() != 40755 {
               break;
        }
    }

    println!("{}", current.get(&'t').unwrap());
}
