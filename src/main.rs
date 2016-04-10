extern crate itertools;
extern crate num;
extern crate permutohedron;
extern crate chrono;

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
mod euler11;
mod euler12;
mod euler13;
mod euler14;
mod euler15;
mod euler16;
mod euler17;
mod euler18;
mod euler19;
mod euler20;
mod euler21;
mod euler22;
mod euler23;
mod euler24;
mod euler25;
mod euler26;
mod euler27;
mod euler30;
mod euler34;
mod euler35;
mod euler36;
mod euler37;
mod euler41;
mod euler42;
mod euler43;
mod euler44;
mod euler45;
mod euler46;
mod euler48;
mod euler49;
mod euler50;
mod euler67;
mod euler52;
mod euler74;
mod euler79;
mod euler92;
mod euler92b;

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
            "11" => euler11::main(),
            "12" => euler12::main(),
            "13" => euler13::main(),
            "14" => euler14::main(),
            "15" => euler15::main(),
            "16" => euler16::main(),
            "17" => euler17::main(),
            "18" => euler18::main(),
            "19" => euler19::main(),
            "20" => euler20::main(),
            "21" => euler21::main(),
            "22" => euler22::main(),
            "23" => euler23::main(),
            "24" => euler24::main(),
            "25" => euler25::main(),
            "26" => euler26::main(),
            "27" => euler27::main(),
            "30" => euler30::main(),
            "34" => euler34::main(),
            "35" => euler35::main(),
            "36" => euler36::main(),
            "37" => euler37::main(),
            "41" => euler41::main(),
            "42" => euler42::main(),
            "43" => euler43::main(),
            "44" => euler44::main(),
            "45" => euler45::main(),
            "46" => euler46::main(),
            "48" => euler48::main(),
            "49" => euler49::main(),
            "50" => euler50::main(),
            "52" => euler52::main(),
            "67" => euler67::main(),
            "74" => euler74::main(),
            "79" => euler79::main(),
            "92" => euler92::main(),
            "92b" => euler92b::main(),
            _ => println!("Hasn't solved yet"),
        }
    }
}
