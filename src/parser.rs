use std::collections::HashSet;

#[derive(Debug, Clone)]
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
