use std::collections::VecDeque;
use std::iter::FromIterator;

fn  array2num(numbers: Vec<u32>) -> u64 {
    numbers.iter()
           .rev()
           .enumerate()
           .fold(0, |acc, (i, &n)| acc + 10_u64.pow(i as u32) * n as u64)
}

#[derive(Clone)]
struct Rotator<T: Clone> {
    queue: VecDeque<T>,
    rotations: usize
}

impl<T: Clone> Rotator<T> {
    fn new(v: Vec<T>) -> Rotator<T> {
        let rot = v.len();
        Rotator{
            queue: VecDeque::from_iter(v.into_iter()),
            rotations: rot
        }
    }
}

impl<T: Clone> Iterator for Rotator<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Vec<T>> {
        if self.rotations == 0 {
            return None;
        }
        let front = self.queue.pop_front().unwrap();
        self.queue.push_back(front);
        self.rotations -= 1;
        let v = self.queue.clone().into_iter().collect::<Vec<_>>();
        Some(v)
    }
}


pub fn main() {
    let primes = (2 as u64..).filter(|&n| (2..(n as f32).sqrt() as u32 + 1).all(|i| n % i as u64 != 0));
    let primes_below1m = primes.take_while(|&n| n <  1_000_000).collect::<Vec<_>>();
    let banned = ['0', '2', '4', '5', '6', '8'];
    let validrotates = |n: &u64| {
        let chars = n.to_string().chars().collect::<Vec<_>>();
        if chars.len() > 1 && banned.iter().any(|n| chars.contains(n)) {
            return false;
        }
        let rotations = Rotator::new(chars);
        rotations.map(|r| array2num(r.iter().map(|&c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()))
                 .all(|p| primes_below1m.contains(&p))
    };
    let ans = primes_below1m.iter().filter(|&p| validrotates(p)).count();
    println!("{}", ans);
}
