use std::char;


fn prime_family_size(p: &str, mask: &[u8]) -> u32 {
    let mut count = 0;
    let isprime = |n: &u64| (2..(*n as f32).sqrt() as u64 + 1).all(|i| n % i != 0);
    for i in 0..10 {
        let c = p.chars().zip(mask.iter()) // zip number with mask
                 .map(|(p_i, &m)| if m == 1 {
                     char::from_digit(i, 10).unwrap()
                 } else {
                     p_i
                 })
                 .collect::<String>();
        if isprime(&c.parse::<u64>().unwrap()) && c.chars().nth(0).unwrap() != '0' {
                count += 1;
        }
    }
    count
}

pub fn main() {
    let primes = (2..).filter(|&n| (2..(n as f32).sqrt() as u64 + 1).all(|i| n % i != 0));
    let penta_patterns = vec![
        [0,1,1,1,0],
        [1,0,1,1,0],
        [1,1,0,1,0],
        [1,1,1,0,0],
    ];
    let hexa_patterns = vec![
        [1,1,1,0,0,0],
        [0,1,1,1,0,0],
        [0,0,1,1,1,0],
        [1,0,0,1,1,0],
        [0,1,0,1,1,0],
        [1,1,0,0,1,0],
        [0,1,1,0,1,0],
        [1,0,1,1,0,0],
        [1,0,1,0,1,0],
    ];


    // is the first 3 elements of a vector the same
    let same = |n: &Vec<char>| n[0] == n[1] && n[1] == n[2];

    let get_masked = |n: &str, mask: &[u8]| {
        n.chars().zip(mask.iter()).filter(|&(_, &m)| m == 1).map(|(n, _)| n).collect::<Vec<_>>()
    };

    let long_primes = primes.skip_while(|&x| x < 10_000)
                            .take_while(|&x| x < 1_000_000);
    for p in long_primes {
        let p = p.to_string();
        if p.len() == 5 {
            for m in &penta_patterns {
                if same(&get_masked(&p, m)) && prime_family_size(&p, m) > 7 {
                    println!("{}", p);
                    return;
                }
            }
        } else {
            for m in &hexa_patterns {
                if same(&get_masked(&p, m)) && prime_family_size(&p, m) > 7 {
                    println!("{}", p);
                    return;
                }
            }
        };
    }
}
