use common::digit_fact_sum;

fn fill(cache: &mut Vec<usize>, i: usize) -> usize {
    let mut chain = vec![i];
    let mut next = i;
    loop {
        next = digit_fact_sum(next);
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
