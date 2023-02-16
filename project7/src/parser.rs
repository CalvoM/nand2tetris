use crate::CommandType;
use std::fs::File;

pub struct Parser {}
impl Parser {
    pub fn new(input_file: File) -> Self {
        unimplemented!()
    }
    pub fn hasMoreLines() -> bool {
        unimplemented!()
    }
    pub fn advance(&mut self) {}
    fn commandType() -> CommandType {
        unimplemented!()
    }
    fn arg1(&self) -> String {
        unimplemented!()
    }
    fn arg2(&self) -> u64 {
        unimplemented!()
    }
}
