use std::collections::HashSet;
use std::error::Error;
use std::fmt;

use crate::lexer::{self, Token};

#[derive(Debug, Clone, Default)]
pub struct Program {
    pub compiler_name: String,
    pub compiler_version: i32,
    pub ireps: Vec<Irep>,
}

#[derive(Debug, Clone)]
pub struct Irep {
    pub name: String,
    pub ops: Vec<Op>,
    pub symbol_pool: HashSet<SymbolMarker>,
    pub string_pool: HashSet<StringMarker>,
}

#[derive(Debug, Clone)]
pub struct Op {
    pub opcode: OpCode,
    pub operand: Vec<Operand>,

    pub label: Option<Label>,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OpCode {
    SSend = 45,
    Return = 56,
    Stirng = 81,
    Stop = 105,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operand {
    Register(i8),
    NumImmdiate(i32),
    SymbolRef(SymbolMarker),
    StringRef(StringMarker),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct StringMarker(String);

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct SymbolMarker(String);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Label {
    name: String,
}

#[derive(Debug)]
pub struct ParseError {}

impl ParseError {
    pub fn raise() -> Self {
        Self {}
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseError.",)
    }
}

impl Error for ParseError {}

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<lexer::Pos>,
    start: usize,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<lexer::Pos>) -> Self {
        Self {
            tokens,
            start: 0,
            current: 0,
        }
    }

    pub fn parse(&self) -> Result<Program, ParseError> {
        let mut program = Program {
            ..Default::default()
        };

        self.generate_program(&program)?;

        Ok(program)
    }

    fn generate_program(&self, mut program: &Program) -> Result<(), ParseError> {
        Ok(())
    }

    fn getcurrent(&mut self, nth: usize) -> Result<Pos, ParseError> {
        let pos = self
            .tokens
            .iter()
            .nth::<Pos>(nth)
            .ok_or_else(Err(ParseError::raise()));
        pos?.clone()
    }

    fn advance(&mut self) -> Result<char, LexError> {
        let c = self.getchar(self.current as usize)?;
        self.current += 1;
        Ok(c)
    }

    fn peek(&mut self) -> Result<char, LexError> {
        self.peek_n(0)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
