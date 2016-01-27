pub fn main() {
    for i in 1..998 {
        for j in i..1000-i {
            let k = 1000 - i - j;
            if k < j { continue }
            if k*k == i*i + j*j {
                println!("{}", i*j*k);
            }
        }
    }
}