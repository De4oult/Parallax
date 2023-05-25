mod interpreter;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    interpreter::parse(args[1].clone());
}
