use project7::{CodeWriter, Parser};
use std::env;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    let input_file_name = &args[1];
    let root_file_name = input_file_name.split('.').collect::<Vec<&str>>()[0];
    let output_file_name = format!("{}{}", root_file_name, ".asm");
    let mut parser = Parser::new(input_file_name);
    let res = parser.parse();
    let mut writer = CodeWriter::new(&output_file_name);
    writer.write(res);
}
