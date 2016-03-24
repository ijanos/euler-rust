fn ispalindrome(n: u32) -> bool {
    let s = n.to_string();
    s.chars().zip(s.chars().rev()).all(|(x, y)| x == y)
}

fn wordvalue(word: &str) -> u32 {
    unimplemented!()
}


fn array2num(numbers: &[char]) -> u32 {
    let mut buf = String::with_capacity(numbers.len());
    for n in numbers.iter() {
        buf.push(*n);
    }
    buf.parse().unwrap()
}

pub fn digit_fact_sum(n: usize) -> usize {
    n.to_string()
     .chars()
     .map(|c| {
         match c {
             '0' => 1,
             '1' => 1,
             '2' => 2,
             '3' => 6,
             '4' => 24,
             '5' => 120,
             '6' => 720,
             '7' => 5040,
             '8' => 40320,
             '9' => 362880,
             _ => panic!(),
         }
     })
     .fold(0, |sum, i| sum + i)
}

pub fn array_to_num(numbers: &[char]) -> u64 {
    numbers.iter().cloned().collect::<String>().parse().unwrap()
}
