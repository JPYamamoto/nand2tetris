// use std::fmt;

use super::label::Label;
use super::instructions::Instruction;

pub type Symbol = String;

pub enum Token {
    Label(Label),
    Instruction(Instruction),
}

// impl fmt::Display for Token {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Token::Label(label) => write!(f, "{}", label),
//             Token::Instruction(instruction) => write!(f, "{}", instruction),
//         }
//     }
// }

pub fn parse_symbol(symbol: &str) -> Option<Symbol> {
    let valid_chars = symbol.chars().all(|c| {
        c.is_ascii_alphanumeric()
            || c == '_'
            || c == '.'
            || c == '$'
            || c == ':'
    });

    let valid_first = symbol.len() > 0
        && !symbol.chars().nth(0).unwrap().is_ascii_digit();

    if valid_chars && valid_first {
        Some(symbol.to_string())
    } else {
        None
    }
}
