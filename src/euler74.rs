fn factsum(n: u32) -> u32 {
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
     .fold(0, |x, acc| acc + x)
}

fn fill(cache: &mut Vec<u32>, i: u32) -> u32 {
    let mut length = 1;
    let mut chain = vec![i];
    let mut next = i;
    loop {
        next = factsum(next);
        if chain.contains(&next) {
            break;
        } else if cache.len() > next as usize && cache[next as usize] != 0 {
            length += cache[next as usize];
            break;
        } else {
            chain.push(next);
            length += 1;
        }
    }
    length
}

pub fn main() {
    const MAX: u32 = 1_000_000;
    let mut cache = vec![0; MAX as usize];
    let mut count = 0;
    for i in 0..MAX {
        let chainlength = fill(&mut cache, i);
        cache[i as usize] = chainlength;
        if chainlength == 60 {
            count += 1;
        }
    }
    println!("{}", count);
}
