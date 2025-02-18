use crate::*;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct MacroContent {
    pub name: String,
    pub args: Vec<FullArgument>,
    pub tokens: Vec<TokenKind>,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct FullArgument {
    pub name: String,
    pub arg_type: ArgumentType,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum ArgumentType {
    Mem,
    Imem,
    Ireg,
    Imm,
    Reg,
}

impl ArgumentType {
    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "mem" => Some(ArgumentType::Mem),
            "imem" => Some(ArgumentType::Imem),
            "ireg" => Some(ArgumentType::Ireg),
            "imm" => Some(ArgumentType::Imm),
            "reg" => Some(ArgumentType::Reg),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum InstructionArgument {
    Mem(i64),
    IMem(i64),
    Reg(i64),
    IReg(i64),
    Imm(i64),
    Ident(String),
    MacroIdent(String),
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct InstructionData {
    pub name: String,
    pub args: Vec<InstructionArgument>,
}
