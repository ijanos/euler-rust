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

struct Fib {
   curr: u32,
   next: u32
}

impl Iterator for Fib {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let new = self.curr + self.next;

        self.curr = self.next;
        self.next = new;

        Some(self.curr)
    }
}

fn euler2() {
    let fib = Fib {curr: 1, next: 1};
    println!("{}", fib
            .take_while(|&a| a < 4_000_000)
            .filter(|&a| a % 2 == 0)
            .fold(0, |sum, i| sum + i));
}

fn main() {
    problem1a();
    problem1b();
    problem1c();
    euler2();
}
