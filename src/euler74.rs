fn factsum(n: usize) -> usize {
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

fn fill(cache: &mut Vec<usize>, i: usize) -> usize {
    let mut chain = vec![i];
    let mut next = i;
    loop {
        next = factsum(next);
        if chain.contains(&next) {
            return chain.len();
        } else if cache.len() > next && cache[next] != 0 {
            return chain.len() + cache[next];
        } else {
            chain.push(next);
        }
    }
}

pub fn main() {
    const MAX: usize = 1_000_000;
    let mut cache: Vec<usize> = vec![0; MAX];
    let mut count = 0;
    for i in 0..MAX {
        let chainlength = fill(&mut cache, i);
        cache[i] = chainlength;
        if chainlength == 60 {
            count += 1;
        }
    }
    println!("{}", count);
}
