use std::env;

mod euler1;
mod euler2;
mod euler3;
mod euler4;
mod euler5;
mod euler6;

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
            _ => println!("Hasn't solved yet")
        }
    }
}
