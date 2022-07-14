use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::symbol_table::SymbolTable;

use super::token::{parse_symbol, Symbol};

const JUMP_VALUES: &'static [&'static str] = &["", "JGT", "JEQ", "JGE", "JLT", "JNE", "JLE", "JMP"];
const DEST_VALUES: &'static [&'static str] = &["", "M", "D", "MD", "A", "AM", "AD", "AMD"];

lazy_static! {
    static ref COMP_VALUES: HashMap<&'static str, u16> = {
        let mut map = HashMap::new();

        map.insert("0",   0b101010);
        map.insert("1",   0b111111);
        map.insert("-1",  0b111010);
        map.insert("D",   0b001100);
        map.insert("A",   0b110000); // M
        map.insert("!D",  0b001101);
        map.insert("!A",  0b110001); // M
        map.insert("-D",  0b001111);
        map.insert("-A",  0b110011); // M
        map.insert("D+1", 0b011111);
        map.insert("A+1", 0b110111); // M
        map.insert("D-1", 0b001110);
        map.insert("A-1", 0b110010); // M
        map.insert("D+A", 0b000010); // M
        map.insert("D-A", 0b010011); // M
        map.insert("A-D", 0b000111); // M
        map.insert("D&A", 0b000000); // M
        map.insert("D|A", 0b010101); // M
        map
    };
}

pub trait Code {
    fn encode(&self, symbol_table: &SymbolTable) -> Option<String>;
}

// ENUMS

pub enum Value {
    Variable(Symbol),
    Constant(u16),
}

pub enum Instruction {
    A(AInstruction),
    C(CInstruction),
}

impl Code for Instruction {
    fn encode(&self, symbol_table: &SymbolTable) -> Option<String> {
        match self {
            Instruction::A(ainstr) => ainstr.encode(symbol_table),
            Instruction::C(cinstr) => cinstr.encode(symbol_table),
        }
    }
}

// STRUCTS

pub struct AInstruction {
    pub value: Value,
}

pub struct CInstruction {
    pub comp: u16,
    pub dest: u16,
    pub jump: u16,
}

// CONSTRUCTORS

impl AInstruction {
    pub fn new(value: Value) -> Self {
        Self { value }
    }

    pub fn parse(code: &str) -> Option<Self> {
        parse_symbol(code)
            .map(|symbol| Value::Variable(symbol))
            .or_else(|| {
                code.parse::<u16>()
                    .ok()
                    .and_then(|opcode| (0x8000 & opcode == 0).then(|| opcode))
                    .map(|constant| Value::Constant(constant))
            })
            .map(|value| AInstruction::new(value))
    }
}

impl CInstruction {
    pub fn new(comp: u16, dest: u16, jump: u16) -> Self {
        Self { comp, dest, jump }
    }

    pub fn parse(code: &str) -> Option<Self> {
        let (dest, rest) = match code.split_once('=') {
            Some(result) => result,
            None         => ("", code),
        };

        let (comp, jump) = match rest.split_once(';') {
            Some(result) => result,
            None         => (rest, ""),
        };

        match [CInstruction::parse_comp(comp), CInstruction::parse_dest(dest), CInstruction::parse_jump(jump)]
            .into_iter().collect::<Option<Vec<_>>>().as_deref() {
                Some([comp, dest, jump]) => Some(CInstruction::new(*comp, *dest, *jump)),
                _                        => None,
            }
    }

    fn parse_jump(code: &str) -> Option<u16> {
        JUMP_VALUES.iter().position(|&j| j == code).map(|p| p as u16)
    }

    fn parse_dest(code: &str) -> Option<u16> {
        DEST_VALUES.iter().position(|&j| j == code).map(|p| (p as u16) << 3)
    }

    fn parse_comp(code: &str) -> Option<u16> {
        let mut a: u16 = 0;

        if code.contains('M') {
            a = 0x1000;
        }

        let code = code.replace('M', "A");
        COMP_VALUES.get(&(code.as_ref())).map(|&v| a | (v << 6))
    }
}

// CODE

impl Code for AInstruction {
    fn encode(&self, symbol_table: &SymbolTable) -> Option<String> {
        match &self.value {
            Value::Variable(variable) => {
                symbol_table.get_address(variable.to_string()).map(|address| {
                    format!("{:016b}", address)
                })
            },
            Value::Constant(constant) => {
                Some(format!("{:016b}", constant))
            }
        }
    }
}

impl Code for CInstruction {
    fn encode(&self, _symbol_table: &SymbolTable) -> Option<String> {
        let opcode = 0xe000 | self.comp | self.dest | self.jump;
        Some(format!("{:016b}", opcode))
    }
}
