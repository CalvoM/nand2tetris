use crate::{CommandType, ParserResult};
use std::{fs::File, io::Write};
pub struct CodeWriter {
    file: File,
}
impl CodeWriter {
    pub fn new(output_file: &str) -> Self {
        Self {
            file: File::create(output_file).unwrap(),
        }
    }
    pub fn write(&mut self, parser_results: Vec<ParserResult>) {
        for result in parser_results {
            match result.command {
                CommandType::C_ARITHMETIC => self.writeArithmetic(result.args[0].clone()),
                CommandType::C_PUSH | CommandType::C_POP => self.writePushPop(
                    result.command,
                    result.args[0].clone(),
                    result.args[1].parse::<u64>().unwrap(),
                ),
                _ => todo!(),
            }
        }
    }
    fn writeArithmetic(&mut self, command: String) {
        let cmd = format!("// {}\r\n", command);
        self.file.write(cmd.as_bytes()).unwrap();
    }
    fn writePushPop(&mut self, command: CommandType, segment: String, index: u64) {
        let cmd = format!("// {:?} {} {}\r\n", command, segment, index);
        self.file.write(cmd.as_bytes()).unwrap();
    }
    fn close(&mut self) {}
}
