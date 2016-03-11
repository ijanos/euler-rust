const MAX: usize = 10_000_000;


fn fill(array: &mut Vec<u8>, index: usize) -> u8 {
    let next = index.to_string()
                    .chars()
                    .map(|c| {
                        match c {
                            '0' => 0,
                            '1' => 1,
                            '2' => 4,
                            '3' => 9,
                            '4' => 16,
                            '5' => 25,
                            '6' => 36,
                            '7' => 49,
                            '8' => 64,
                            '9' => 81,
                            _ => panic!(),
                        }
                    })
                    .fold(0, |acc, n| acc + n);
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
        if numbers[i] != 0 {
            continue;
        }
        numbers[i] = fill(&mut numbers, i);
    }
    println!("{}", numbers.iter().filter(|&&n| n == 89).count());
}
