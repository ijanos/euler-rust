use std::char;


fn prime_family(p: &str, mask: &[u8]) -> u32 {
    let mut count = 0;
    let isprime = |n: &u64| (2..(*n as f32).sqrt() as u64 + 1).all(|i| n % i != 0);
    for i in 0..10 {
        let c = p.chars()
                 .zip(mask.iter())
                 .map(|(pi, &m)| if m == 1 {
                     char::from_digit(i, 10).unwrap()
                 } else {
                     pi
                 })
                 .collect::<String>();
        if isprime(&c.parse::<u64>().unwrap()) {
            if c.chars().nth(0).unwrap() != '0' {
                count += 1;
            }
        }
    }
    count
}

pub fn main() {
    let primes = || (2..).filter(|&n| (2..(n as f32).sqrt() as u64 + 1).all(|i| n % i != 0));
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


    let same = |n: &Vec<char>| n[0] == n[1] && n[1] == n[2];
    let get_masked = |n: &str, mask: &[u8]| {
        n.chars().zip(mask.iter()).filter(|&(_, &m)| m == 1).map(|(n, _)| n).collect::<Vec<_>>()
    };

    let ten_thousands = primes()
                            .skip_while(|&x| x < 10_000)
                            .take_while(|&x| x < 100_000)
                            .collect::<Vec<u64>>();
    for p in ten_thousands {
        let p = p.to_string();
        for m in &penta_patterns {
            if same(&get_masked(&p, m)) {
                if prime_family(&p, m) > 7 {
                    println!("{}", p);
                    return;
                }
            }
        }
    }

    let hundred_thousands = primes()
                                .skip_while(|&x| x < 100_000)
                                .take_while(|&x| x < 1_000_000)
                                .collect::<Vec<u64>>();
    for &p in &hundred_thousands {
        let p = p.to_string();
        for m in &hexa_patterns {
            if same(&get_masked(&p, m)) {
                if prime_family(&p, m) > 7 {
                    println!("{}", p);
                    return;
                }
            }
        }
    }
}
