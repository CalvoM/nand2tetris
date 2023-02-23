use project7::Parser;
use std::env;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    let input_file_name = &args[1];
    let root_file_name = input_file_name.split('.').collect::<Vec<&str>>()[0];
    let output_file_name = format!("{}{}", root_file_name, ".asm");
    let mut parser = Parser::new(input_file_name);
    parser.parse();
    parser.advance();
    println!("More {}", parser.hasMoreLines());
    parser.advance();
    println!("More {}", parser.hasMoreLines());
    parser.advance();
    println!("More {}", parser.hasMoreLines());
    parser.advance();
    println!("More {}", parser.hasMoreLines());
}
