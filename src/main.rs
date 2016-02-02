extern crate num;
use std::env;

mod euler1;
mod euler2;
mod euler3;
mod euler4;
mod euler5;
mod euler6;
mod euler7;
mod euler8;
mod euler9;
mod euler10;
mod euler12;
mod euler14;
mod euler15;
mod euler20;
mod euler52;
mod euler92;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Provide a number as parameter");
    } else {
        match args[1].as_ref() {
            "1" => euler1::main(),
            "2" => euler2::main(),
            "3" => euler3::main(),
            "4" => euler4::main(),
            "5" => euler5::main(),
            "6" => euler6::main(),
            "7" => euler7::main(),
            "8" => euler8::main(),
            "9" => euler9::main(),
            "10" => euler10::main(),
            "12" => euler12::main(),
            "14" => euler14::main(),
            "15" => euler15::main(),
            "20" => euler20::main(),
            "52" => euler52::main(),
            "92" => euler92::main(),
            _ => println!("Hasn't solved yet")
        }
    }
}
