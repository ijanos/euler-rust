use std::io;
use std::io::prelude::*;
use std::collections::HashMap;


pub fn main() {
    let value: HashMap<char, usize> = (b'A'..b'Z' + 1)
                                          .enumerate()
                                          .map(|(i, c)| (c as char, i + 1))
                                          .collect();

    println!("Pipe the puzzle input to stdin");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap().replace("\"", "");
        let mut names: Vec<&str> = line.split(",").collect();
        names.sort();
        println!("{}",
                 names.iter()
                      .enumerate()
                      .map(|(i, name)| {
                          (i + 1) *
                          name.chars()
                              .map(|c| value.get(&c).unwrap())
                              .fold(0, |acc, i| acc + i)
                      })
                      .fold(0, |acc, i| acc + i));
    }
}
