use crate::{CommandType, ParserResult};
use std::{collections::HashMap, fs::File, io::Write};

const X_VAR: &str = "@R14";
const Y_VAR: &str = "@R15";
const TEMP_VAR: &str = "@R13";
const TEMP_LOC_BASE: usize = 5;
const STATIC_LOC_BASE: usize = 16;
pub struct CodeWriter {
    file: File,
    segments: HashMap<String, String>,
    line_number: usize,
    comp_line: String,
    end_comp_line: String,
}
impl CodeWriter {
    pub fn new(output_file: &str) -> Self {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert(String::from("local"), String::from("@LCL"));
        map.insert(String::from("argument"), String::from("@ARG"));
        map.insert(String::from("this"), String::from("@THIS"));
        map.insert(String::from("that"), String::from("@THAT"));
        Self {
            file: File::create(output_file).unwrap(),
            segments: map,
            line_number: 0,
            comp_line: String::new(),
            end_comp_line: String::new(),
        }
    }
    pub fn write(&mut self, parser_results: Vec<ParserResult>) {
        for result in parser_results {
            match result.command {
                CommandType::C_ARITHMETIC => self.writeArithmetic(result.args[0].clone()),
                CommandType::C_PUSH | CommandType::C_POP => self.writePushPop(
                    result.command,
                    result.args[0].clone(),
                    result.args[1].parse::<usize>().unwrap(),
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
                self.set_y();
                self.decr_sp();
                self.set_x();
            }
            "neg" | "not" => {
                self.set_y();
            }
            _ => todo!(),
        }
        match command.as_str() {
            "add" | "sub" | "and" | "or" => {
                self.get_x();
                self.add_sub_and_or(&command);
            }
            "neg" | "not" => {
                self.get_y();
                self.neg_not(&command);
            }
            "gt" | "lt" | "eq" => {
                self.write_default_set_false();
                self.get_x();
                self.logical_cmd(&command);
            }
            _ => todo!(),
        }
        self.add_value_to_stack();
        self.incr_sp();
    }
    fn writePushPop(&mut self, command: CommandType, segment: String, index: usize) {
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
            CommandType::C_POP => {
                self.decr_sp();
                self.handle_pop_segment(segment.as_str(), index);
            }
            CommandType::C_PUSH => {
                self.handle_push_segment(segment.as_str(), index);
                self.incr_sp()
            }
            _ => todo!(),
        }
    }
    fn handle_pop_segment(&mut self, segment: &str, index: usize) {
        match segment {
            "constant" => {
                //self.push_constant(index);
            }
            "local" | "argument" | "this" | "that" => {
                // self.push_reg_values(segment, index);
                self.pop_reg_values(segment, index);
            }
            "temp" | "static" => {
                // self.push_reg_range(segment, index);
            }
            "pointer" => {
                self.get_value_from_stack();
                self.pop_pointer(index);
            }
            _ => todo!(),
        }
    }
    fn handle_push_segment(&mut self, segment: &str, index: usize) {
        match segment {
            "constant" => {
                self.push_constant(index);
            }
            "local" | "argument" | "this" | "that" => {
                self.push_reg_values(segment, index);
            }
            "temp" | "static" => {
                self.push_reg_range(segment, index);
            }
            "pointer" => {
                self.push_pointer(index);
            }
            _ => todo!(),
        }
        self.add_value_to_stack();
    }
    fn pop_pointer(&mut self, index: usize) {
        let ram_loc = match index {
            0 => "@R3",
            1 => "@R4",
            _ => todo!(),
        };
        self.write_cmd(&mut [ram_loc, "M=D"]);
    }
    fn push_pointer(&mut self, index: usize) {
        let ram_loc = match index {
            0 => "@R3",
            1 => "@R4",
            _ => todo!(),
        };
        self.write_cmd(&mut [ram_loc, "D=M"]);
    }
    fn pop_reg_range(&mut self, reg: &str, index: usize) {}
    fn push_reg_range(&mut self, reg: &str, index: usize) {
        let loc = match reg {
            "temp" => TEMP_LOC_BASE + index,
            "static" => STATIC_LOC_BASE + index,
            _ => todo!(),
        };
        self.write_cmd(&mut [format!("@{}", loc).as_str(), "D=M"]);
    }
    fn pop_reg_values(&mut self, reg: &str, index: usize) {
        let seg = self.segments.get_mut(reg).unwrap().clone();
        self.write_cmd(&mut [
            format!("@{}", index).as_str(),
            "D=A",
            seg.as_str(),
            "D=D+M",
            TEMP_VAR,
            "M=D",
        ]);
        self.get_value_from_stack();
        self.write_cmd(&mut [TEMP_VAR, "A=M", "M=D"]);
    }
    fn push_reg_values(&mut self, reg: &str, index: usize) {
        let seg = self.segments.get_mut(reg).unwrap().clone();
        self.write_cmd(&mut [
            format!("@{}", index).as_str(),
            "D=A",
            seg.as_str(),
            "D=D+M",
            "A=D",
            "D=M",
        ]);
    }
    fn push_constant(&mut self, index: usize) {
        self.write_cmd(&mut ["//Push Constant", format!("@{}", index).as_str(), "D=A"]);
    }
    fn write_set_true(&mut self) {
        self.write_cmd(&mut [
            "//Set true",
            format!("({})", self.comp_line.clone().as_str()).as_str(),
            TEMP_VAR,
            "M=-1",
            format!("({})", self.end_comp_line).as_str(),
        ])
    }
    fn add_value_to_stack(&mut self) {
        // sure value is in D reg
        self.write_cmd(&mut ["@SP", "A=M", "M=D"]);
    }
    fn get_value_from_stack(&mut self) {
        self.write_cmd(&mut ["@SP", "A=M", "D=M"])
    }
    fn write_default_set_false(&mut self) {
        self.write_cmd(&mut ["//Set false", TEMP_VAR, "M=0"])
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
    fn add_sub_and_or(&mut self, op: &str) {
        let sign = match op {
            "add" => "+",
            "sub" => "-",
            "and" => "&",
            "or" => "|",
            _ => "+",
        };
        self.write_cmd(&mut [Y_VAR, format!("D=D{}M", sign).as_str()]);
    }
    fn neg_not(&mut self, op: &str) {
        let mut sign = "-";
        if op == "not" {
            sign = "!";
        }
        self.write_cmd(&mut [format!("D={}D", sign).as_str()]);
    }
    fn logical_cmd(&mut self, op: &str) {
        let jmp = match op {
            "eq" => "JEQ",
            "gt" => "JGT",
            "lt" => "JLT",
            _ => todo!(),
        };
        self.comp_line = format!("SET_TRUE_{}", self.line_number);
        self.end_comp_line = format!("END_TRUE_{}", self.line_number);
        self.write_cmd(&mut [
            Y_VAR,
            "D=D-M",
            format!("@{}", self.comp_line).as_str(),
            format!("D;{}", jmp).as_str(),
            format!("@{}", self.end_comp_line).as_str(),
            "0;JMP",
        ]);
        self.write_set_true();
        //TODO: we will have to find how to push better
        self.write_cmd(&mut [TEMP_VAR, "D=M"]);
    }
    fn write_cmd(&mut self, cmd_args: &mut [&str]) {
        let mut cmd = cmd_args.join("\r\n");
        cmd.push_str("\r\n");
        self.line_number += cmd_args.len();
        self.file.write(cmd.as_bytes()).unwrap();
    }
    fn close(&mut self) {}
}
