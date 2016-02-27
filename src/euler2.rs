struct Fib {
    curr: u32,
    next: u32,
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

pub fn main() {
    let fib = Fib { curr: 1, next: 1 };
    println!("{}",
             fib.take_while(|&a| a < 4_000_000)
                .filter(|&a| a % 2 == 0)
                .fold(0, |sum, i| sum + i));
}
