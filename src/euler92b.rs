const MAX: usize = 10_000_000;

fn fill(array: &mut Box<[u8; MAX]>, index: usize) -> u8 {
    let digits: Vec<u32> = index.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect();
    let next = digits.iter().fold(0, |acc, n| acc + n*n) as usize;
    if array[next] == 0 {
        array[next] = fill(array, next);
    }
    return array[next];
}

pub fn main() {
    let mut numbers: Box<[u8; MAX]> = Box::new([0; MAX]);
    numbers[1] = 1;
    numbers[89] = 89;
    for i in 2..MAX {
        numbers[i] = fill(&mut numbers, i);
    }
    println!("{}", numbers.iter().filter(|&n| *n == 89).count());
}