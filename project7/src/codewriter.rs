use crate::{CommandType, ParserResult};
use std::{collections::HashMap, fs::File, io::Write};

const X_VAR: &str = "@R14";
const Y_VAR: &str = "@R15";
pub struct CodeWriter {
    file: File,
    segments: HashMap<String, String>,
}
impl CodeWriter {
    pub fn new(output_file: &str) -> Self {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert(String::from("local"), String::from("LCL"));
        map.insert(String::from("argument"), String::from("ARG"));
        map.insert(String::from("this"), String::from("THIS"));
        map.insert(String::from("that"), String::from("THAT"));
        Self {
            file: File::create(output_file).unwrap(),
            segments: map,
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
        // Var x stored in R14
        // Var y stored in R15
        let cmd = format!("// {}\r\n", command);
        self.file.write(cmd.as_bytes()).unwrap();
        self.decr_sp();
        match command.as_str() {
            "add" | "sub" | "eq" | "gt" | "lt" | "and" | "or" => {
                self.set_x();
                self.decr_sp();
                self.set_y();
            }
            "neg" | "not" => {
                self.set_y();
            }
            _ => todo!(),
        }
        match command.as_str() {
            "add" | "sub" => {
                self.get_x();
                self.add_or_sub(&command);
            }
            "neg" | "not" => {
                self.get_y();
                self.neg_or_not(&command);
                self.incr_sp();
            }
            _ => todo!(),
        }
    }
    fn writePushPop(&mut self, command: CommandType, segment: String, index: u64) {
        // PUSH
        // get SP
        // add item to stack
        // incr SP
        // POP
        // decr SP
        // get item from stack
        let cmd = format!("// {:?} {} {}\r\n", command, segment, index);
        self.file.write(cmd.as_bytes()).unwrap();
        match command {
            CommandType::C_POP => self.decr_sp(),
            CommandType::C_PUSH => self.incr_sp(),
            _ => todo!(),
        }
    }
    fn incr_sp(&mut self) {
        self.write_cmd(&mut ["// Incr SP", "@SP", "M=M+1"]);
    }
    fn decr_sp(&mut self) {
        self.write_cmd(&mut ["// Decr SP", "@SP", "M=M-1"]);
    }
    fn set_x(&mut self) {
        self.write_cmd(&mut ["// Set x", "@SP", "A=M", "D=M", X_VAR, "M=D"]);
    }
    fn set_y(&mut self) {
        self.write_cmd(&mut ["//Set y", "@SP", "A=M", "D=M", Y_VAR, "M=D"]);
    }
    fn get_x(&mut self) {
        self.write_cmd(&mut ["//Get x", X_VAR, "D=M"]);
    }
    fn get_y(&mut self) {
        self.write_cmd(&mut ["//Get y", Y_VAR, "D=M"]);
    }
    fn add_or_sub(&mut self, op: &str) {
        let mut sign = "+";
        if op == "sub" {
            sign = "-";
        }
        self.write_cmd(&mut [Y_VAR, format!("D=D{}M", sign).as_str(), "@SP", "A=M", "M=D"]);
    }
    fn neg_or_not(&mut self, op: &str) {
        let mut sign = "-";
        if op == "not" {
            sign = "!";
        }
        self.write_cmd(&mut [format!("D={}D", sign).as_str(), "@SP", "A=M", "M=D"]);
    }
    fn write_cmd(&mut self, cmd_args: &mut [&str]) {
        let mut cmd = cmd_args.join("\r\n");
        cmd.push_str("\r\n");
        self.file.write(cmd.as_bytes()).unwrap();
    }
    fn close(&mut self) {}
}
