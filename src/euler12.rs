pub fn main() {
    let mut triangles = (1..).map(|n| (1..n + 1).fold(0, |acc, n| acc + n));
    println!("{}",
             triangles.find(|&x| {
                          (1..(x as f64).sqrt() as u32 + 1).filter(|&i| x % i == 0).count() * 2 >
                          500
                      })
                      .unwrap());
}
