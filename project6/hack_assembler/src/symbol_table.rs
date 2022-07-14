use std::collections::HashMap;

use crate::tokens::Symbol;
use crate::tokens::token::Token;
use crate::tokens::instructions::{Instruction, AInstruction, Value};

const FIRST_SYMBOLS: &'static [&'static str] = &["SP", "LCL", "ARG", "THIS", "THAT"];
const IO_SYMBOLS: &'static [(&'static str, u16)] = &[("SCREEN", 16384), ("KBD", 24576)];

pub type Address = u16;

pub struct SymbolTable {
    symbol_table: HashMap<Symbol, Address>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut symbol_table: HashMap<Symbol, Address> = HashMap::new();

        (0..=15).for_each(|i| { symbol_table.insert(format!("R{}", i), i); });

        FIRST_SYMBOLS.iter().enumerate().for_each(|(i, symbol)| {
            symbol_table.insert(symbol.to_string(), i as u16);
        });

        IO_SYMBOLS.iter().for_each(|(symbol, address)| {
            symbol_table.insert(symbol.to_string(), *address);
        });

        Self { symbol_table }
    }

    pub fn populate(&mut self, tokens: Vec<Token>) -> Vec<Instruction> {
        let mut instructions = vec![];
        let mut count_token = 0;

        for token in tokens {
            match token {
                Token::Label(label) => {
                    self.symbol_table.insert(label.symbol, count_token);
                },
                Token::Instruction(instruction) => {
                    instructions.push(instruction);
                    count_token += 1;
                }
            }

        }

        let mut count_variable = 16;

        for instruction in instructions.iter() {
            if let Instruction::A(AInstruction { value }) = &instruction {
                if let Value::Variable(variable) = value {
                    if !self.symbol_table.contains_key(variable) {
                        self.symbol_table.insert(variable.clone(), count_variable);
                        count_variable += 1;
                    }
                }
            }
        }

        instructions
    }

    pub fn get_address(&self, symbol: Symbol) -> Option<Address> {
        self.symbol_table.get(&symbol).cloned()
    }
}
