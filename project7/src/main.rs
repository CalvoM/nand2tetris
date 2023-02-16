use std::env;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    dbg!(input_file);
}
