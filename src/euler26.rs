use std::iter;

use num::rational::Ratio;
use num::bigint::BigUint;
use num::bigint::ToBigUint;

fn find_repeating(text: &str) -> &str {
    // horrible but works
    let i = (1..text.len() / 2).find(|&i| {
        iter::repeat(&text[0..i]).take(text.len() / i).collect::<String>() ==
        &text[0..text.len() / i * i]
    });
    match i {
        Some(i) => &text[0..i],
        None => "",
    }
}


fn get_decimals(r: Ratio<BigUint>, max_decimals: u32) -> String {
    // this function will skip leading zeroes!
    let mut fract = r.fract();
    let ten = Ratio::new(10.to_biguint().unwrap(), 1.to_biguint().unwrap());
    for _ in 0..max_decimals {
        if fract.is_integer() {
            break;
        }
        fract = fract * &ten;
    }
    fract.trunc().to_string()
}


pub fn main() {
    let primes = (2 as i32..)
                     .filter(|&n| (2..(n as f32).sqrt() as u32 + 1).all(|i| n % i as i32 != 0));
    let primes = primes.take_while(|&x| x < 1_001);

    let mut max_length = 0;
    let mut ans = 0;

    for i in primes {
        let decimals = get_decimals(Ratio::new(1.to_biguint().unwrap(), i.to_biguint().unwrap()),
                                    2000);
        let repeating_length = find_repeating(&decimals).len();
        if repeating_length > max_length {
            max_length = repeating_length;
            ans = i;
        }
    }
    println!("{}", ans);
}
