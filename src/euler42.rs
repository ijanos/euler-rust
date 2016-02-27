use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

pub fn main() {
    let value: HashMap<char, usize> = (b'A'..b'Z' + 1)
                                          .enumerate()
                                          .map(|(i, c)| (c as char, i + 1))
                                          .collect();

    let triangle = (1..).map(|n| n * (n + 1) / 2);
    let first50triangle = triangle.take(50).collect::<Vec<_>>();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap().replace("\"", "");
        let names: Vec<&str> = line.split(",").collect();
        println!("{}",
                 names.iter()
                      .map(|name| {
                          name.chars().map(|c| value.get(&c).unwrap()).fold(0, |acc, i| acc + i)
                      })
                      .filter(|n| first50triangle.contains(n))
                      .count());
    }
}
