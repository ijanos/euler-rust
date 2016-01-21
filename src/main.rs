use std::env;

mod euler1;
mod euler2;
mod euler3;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Provide a number as parameter");
    } else {
        match args[1].as_ref() {
            "1" => euler1::main(),
            "2" => euler2::main(),
            "3" => euler3::main(),
            _ => println!("Hasn't solved yet")
        }
    }
}
