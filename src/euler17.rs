use std::char;

fn say_single_digit(digit: char) -> &'static str {
    match digit {
        '1' => "one",
        '2' => "two",
        '3' => "three",
        '4' => "four",
        '5' => "five",
        '6' => "six",
        '7' => "seven",
        '8' => "eight",
        '9' => "nine",
        '0' => "",
        _ => panic!(),
    }
}

fn say_below_20(num: &str) -> &'static str {
    match num {
        "10" => "ten",
        "11" => "eleven",
        "12" => "twelve",
        "13" => "thirteen",
        "14" => "fourteen",
        "15" => "fifteen",
        "16" => "sixteen",
        "17" => "seventeen",
        "18" => "eighteen",
        "19" => "nineteen",
        _ => panic!(),
    }
}

fn say_tens(num: u32) -> String {
    let nums: Vec<char> = num.to_string().chars().collect();
    if num < 10 {
        return say_single_digit(nums[0]).to_owned();
    }
    if num < 20 {
        return say_below_20(&num.to_string()).to_owned();
    }
    let tens = match nums[0] {
        '2' => "twenty",
        '3' => "thirty",
        '4' => "forty",
        '5' => "fifty",
        '6' => "sixty",
        '7' => "seventy",
        '8' => "eighty",
        '9' => "ninety",
        _ => panic!(),
    };
    let digit = say_single_digit(nums[1]);
    [tens, digit].join("")
}

fn say_below_1000(num: u32) -> Option<String> {
    if num > 999 {
        return None;
    }
    let mut buf = String::new();
    let rest = say_tens(num % 100);
    if num > 99 {
        let hunders = say_single_digit(char::from_digit(num / 100, 10).unwrap());
        buf.push_str(hunders);
        if rest == "" {
            buf.push_str("hundred");
        } else {
            buf.push_str("hundredand");
        }
    }
    buf.push_str(&rest);
    Some(buf)
}

pub fn main() {
    let mut sum = "onethousand".len();
    for i in 1..1000 {
        sum += say_below_1000(i).unwrap().len();
    }
    println!("{}", sum);
}
