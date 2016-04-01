use std::io;
use std::io::prelude::*;

use euler18::expand;

pub fn main() {
    println!("Pipe the puzzle input to stdin");

    let mut lines: Vec<Vec<u32>> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        lines.push(line.unwrap().split_whitespace().map(|num| num.parse::<u32>().unwrap()).collect());
    }

    let mut summed = expand(&lines[0]);
    for line in &lines[1..] {
        summed = expand(&summed.iter().zip(line).map(|(x,y)| x + y).collect());
    }
    println!("{}", summed.iter().max().unwrap());
}
