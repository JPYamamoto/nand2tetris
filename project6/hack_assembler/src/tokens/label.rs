use super::token::{parse_symbol, Symbol};

pub struct Label {
    pub symbol: Symbol,
}

impl Label {
    pub fn new(symbol: String) -> Self {
        Self { symbol }
    }

    pub fn parse(label: &str) -> Option<Self> {
        parse_symbol(label).map(|symbol| Label::new(symbol))
    }
}
