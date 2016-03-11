use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;


pub fn main() {
    let mut edges = HashMap::<char, HashSet<char>>::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let numbers = line.chars().collect::<Vec<_>>();
        {
            let set = edges.entry(numbers[0]).or_insert(HashSet::<char>::new());
            set.insert(numbers[1]);
            set.insert(numbers[2]);
        }
        let set = edges.entry(numbers[1]).or_insert(HashSet::<char>::new());
        set.insert(numbers[2]);
    }
    let mut vec: Vec<_> = edges.iter().collect();
    vec.sort_by_key(|&(_, set)| set.len());
    let mut ans = vec.iter().rev().map(|&(&n, _)| n).collect::<String>();
    let (_ ,last) = vec[0];
    for &c in last {
        ans.push(c);
    }
    println!("{}", ans);
}
