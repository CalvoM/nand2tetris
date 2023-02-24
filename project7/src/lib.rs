pub mod codewriter;
pub mod parser;
pub use codewriter::CodeWriter;
pub use parser::Parser;
pub use parser::ParserResult;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CommandType {
    C_ARITHMETIC,
    C_PUSH,
    C_POP,
    C_LABEL,
    C_GOTO,
    C_IF,
    C_FUNCTION,
    C_RETURN,
    C_CALL,
}
