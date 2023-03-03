use crate::CommandType;
use std::{fs::File, io::Read};

#[derive(Debug)]
pub struct ParserResult {
    pub command: CommandType,
    pub args: Vec<String>,
}

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
    pub fn parse(&mut self) -> Vec<ParserResult> {
        let mut parser_results: Vec<ParserResult> = vec![];
        let mut input_file = File::open(self.file_name.as_str()).unwrap();
        input_file.read_to_string(&mut self.content).unwrap();
        self.lines = self
            .content
            .trim()
            .split('\n')
            .map(|s| s.trim_end().to_string())
            .collect();
        self.line_number += 1;
        while self.hasMoreLines() {
            self.advance();
            parser_results.push(ParserResult {
                command: self.commandType(),
                args: vec![self.arg1(), self.arg2().to_string()],
            })
        }
        parser_results
    }
    pub fn hasMoreLines(&self) -> bool {
        self.lines.len() - 1 >= self.line_number as usize
    }
    pub fn advance(&mut self) {
        while self.hasMoreLines() {
            self.current_cmd = self.lines[self.line_number as usize].clone();
            let cmds: Vec<&str> = self.current_cmd.split(' ').collect();
            self.line_number += 1;
            if cmds[0].starts_with("//") || cmds[0] == "" {
                continue;
            }
            break;
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
    fn arg2(&self) -> i64 {
        let cmds: Vec<&str> = self.current_cmd.split(' ').collect();
        let cmd = self.commandType();
        if cmd == CommandType::C_PUSH
            || cmd == CommandType::C_POP
            || cmd == CommandType::C_FUNCTION
            || cmd == CommandType::C_CALL
        {
            return cmds[2].parse::<i64>().unwrap();
        } else {
            return -1;
        }
    }
}
