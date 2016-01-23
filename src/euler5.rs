fn a(){
    let mut ok;
    for i in 1.. {
        ok = true;
        for j in 10..21 {
            if i % j != 0 {
                ok = false;
                break
            }
        }
        if ok {
            println!("{}", i);
            break;
        }
    }
}

fn b(){
    println!("{}", (20..).filter(|&n| (10..21).all(|i| n % i == 0)).nth(0).unwrap());
}


pub fn main(){
    a();
    b();
}
