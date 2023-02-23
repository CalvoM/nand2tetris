use crate::CommandType;
use std::{fs::File, io::Read};

pub struct Parser {
    line_number: isize,
    lines: Vec<String>,
    file_name: String,
    content: String,
    current_cmd: String,
}
impl Parser {
    pub fn new(input_file_name: &str) -> Self {
        Self {
            line_number: -1,
            lines: vec![],
            file_name: input_file_name.to_owned(),
            content: String::new(),
            current_cmd: String::new(),
        }
    }
    pub fn parse(&mut self) {
        let mut input_file = File::open(self.file_name.as_str()).unwrap();
        input_file.read_to_string(&mut self.content).unwrap();
        self.lines = self.content.split('\n').map(|s| s.to_string()).collect();
    }
    pub fn hasMoreLines(&self) -> bool {
        self.lines.len() != self.line_number as usize
    }
    pub fn advance(&mut self) {
        if self.hasMoreLines() {
            self.line_number += 1;
            self.current_cmd = self.lines[self.line_number as usize].clone();
        }
    }
    fn commandType(&self) -> CommandType {
        let cmds: Vec<&str> = self.current_cmd.split(' ').collect();
        let cmd = match cmds[0] {
            "add" | "sub" | "neg" => CommandType::C_ARITHMETIC,
            "eq" | "gt" | "lt" => CommandType::C_ARITHMETIC,
            "and" | "or" | "not" => CommandType::C_ARITHMETIC,
            "push" => CommandType::C_PUSH,
            "pop" => CommandType::C_POP,
            &_ => todo!(),
        };
        cmd
    }
    fn arg1(&self) -> String {
        let cmds: Vec<&str> = self.current_cmd.split(' ').collect();
        let cmd = self.commandType();
        if cmd == CommandType::C_ARITHMETIC {
            return String::from(cmds[0]);
        } else if cmd == CommandType::C_PUSH || cmd == CommandType::C_POP {
            return String::from(cmds[1]);
        } else {
            unimplemented!()
        }
    }
    fn arg2(&self) -> u64 {
        let cmds: Vec<&str> = self.current_cmd.split(' ').collect();
        let cmd = self.commandType();
        if cmd == CommandType::C_PUSH
            || cmd == CommandType::C_POP
            || cmd == CommandType::C_FUNCTION
            || cmd == CommandType::C_CALL
        {
            return cmds[2].parse::<u64>().unwrap();
        } else {
            unimplemented!()
        }
    }
}
