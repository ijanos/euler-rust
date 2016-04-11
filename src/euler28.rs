use std::iter;

pub fn main() {
    // produces the length of numbers skipped between diagonals on a number spiral
    // 1 1 1 1 3 3 3 3 5 5 5 5 7 etc.
    let skips = (1..).map(|n| 2 * n - 1).flat_map(|i| iter::repeat(i).take(4));

    let mut cur = 1; // current diagonal number
    let mut sum = 1;

    // every ring contains 4 numbers
    // 500 rings produce a 1001x1001 square
    for i in skips.take(4 * 500) {
        cur += i + 1;
        sum += cur;
    }
    println!("{}", sum);
}
