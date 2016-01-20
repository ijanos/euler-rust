fn problem1a() {
    let mut sum = 0;
    for n in 1..1000 {
        if n % 3 == 0 || n % 5 == 0 {
            sum += n
        }
    }
    println!("{}", sum);
}

fn problem1b() {
    println!("{}", (1..1000)
            .filter(|n| n % 3 == 0 || n % 5 == 0)
            .fold(0, |sum, i| sum + i)
    );
}

fn problem1c() {
   println!("{}",
       (1..1000)
        .fold(0, |sum, i| {
                if i % 3 == 0 || i % 5 == 0 { sum + i } else { sum }
            }
       )
   );
}

pub fn main() {
    problem1a();
    problem1b();
    problem1c();
}