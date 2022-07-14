use crate::tokens::token::Token;
use crate::tokens::label::Label;
use crate::tokens::instructions::{AInstruction, CInstruction, Instruction};

pub fn parse(source: String) -> Option<Vec<Token>> {
    source.split('\n')
          .map(|line| remove_comments(line))
          .filter(|line| !line.is_empty())
          .map(|line| parse_line(line))
          .collect::<Vec<Option<Token>>>()
          .into_iter()
          .collect::<Option<Vec<Token>>>()
}

fn parse_line(line: &str) -> Option<Token> {
    if line.starts_with('(') && line.ends_with(')') {
        Label::parse(&line[1..(line.len()-1)]).map(|label| Token::Label(label))
    } else if line.starts_with('@') {
        AInstruction::parse(&line[1..]).map(|ainstr| Token::Instruction(Instruction::A(ainstr)))
    } else {
        CInstruction::parse(line).map(|cinstr| Token::Instruction(Instruction::C(cinstr)))
    }
}

fn remove_comments(line: &str) -> &str {
    line.split_once("//").map_or(line, |(code, _)| code).trim()
}
