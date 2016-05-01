use std::collections::HashSet;
use std::collections::VecDeque;

fn factors_len(n: u32) -> usize {
    let mut n = n;
    let mut i = 2;
    let mut factors = HashSet::<u32>::new();
    while i <= n {
        if n % i == 0 {
            n /= i;
            factors.insert(i);
        } else {
            i += 1;
        }
    }
    factors.len()
}

pub fn main() {
    let mut buf = VecDeque::new();
    buf.push_back((1, factors_len(1)));
    buf.push_back((2, factors_len(2)));
    buf.push_back((3, factors_len(3)));
    buf.push_back((4, factors_len(4)));
    let mut i = 5;
    loop {
        buf.pop_front();
        let factors = factors_len(i);
        buf.push_back((i, factors));
        if buf.iter().all(|&(_, len)| len == 4) {
            println!("{}", buf[0].0);
            break;
        }
        i += if factors != 4 {
            4 // skip 4 numbers
        } else {
            1
        };
    }

    // Alternative solution
    // let numbers = (1..200_000).collect::<Vec<_>>();
    // println!("{:?}", numbers.windows(4).find(|&l| l.iter().all(|&n| factors_len(n) == 4)));
}
