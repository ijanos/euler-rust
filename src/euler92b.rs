const MAX: usize = 10_000_000;

fn fill(array: &mut Vec<u8>, index: usize) -> u8 {
    let next = index
                .to_string()
                .chars()
                .map(|c| {
                    let n = c.to_digit(10).unwrap();
                    n*n
                 })
                 .fold(0, |acc, n| acc + n) as usize;
    if array[next] == 0 {
        array[next] = fill(array, next);
    }
    return array[next];
}

pub fn main() {
    let mut numbers = vec![0; MAX];
    numbers[1] = 1;
    numbers[89] = 89;
    for i in 2..MAX {
        numbers[i] = fill(&mut numbers, i);
    }
    println!("{}", numbers.iter().filter(|&n| *n == 89).count());
}