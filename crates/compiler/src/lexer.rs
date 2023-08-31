use std::rc::Rc;

use crate::{
    error::{Error, Reason, Result},
    symbol::{Symbol, KEYWORDS},
    util::{Quote, Q},
};

pub const NULL: u8 = 0;

pub struct Lexer {
    source: Rc<[u8]>,
    index: usize,
}

/// General lexer functions
impl Lexer {
    pub fn new(source: Rc<[u8]>) -> Self {
        assert!(
            !source.contains(&NULL),
            "Source must not contain a null-character."
        );
        Self { source, index: 0 }
    }

    /// Get the current char
    pub fn get(&self) -> u8 {
        self.source.get(self.index).copied().unwrap_or(NULL)
    }

    /// Get the next char
    pub fn peek(&self) -> u8 {
        self.source.get(self.index + 1).copied().unwrap_or(NULL)
    }

    /// Increment the current index
    pub fn advance(&mut self) {
        self.index += 1;
    }

    /// Quote the symbol
    pub fn quote(&mut self, value: Symbol, start: usize) -> Result<Q<Symbol>> {
        Ok(Q::new(value, start, self.index))
    }

    /// Advance and quote the symbol afterwards
    pub fn quote_next(&mut self, value: Symbol, start: usize) -> Result<Q<Symbol>> {
        self.advance();
        self.quote(value, start)
    }

    /// Create an error with the given reason
    pub fn error(&self, reason: Reason, start: usize) -> Result<Q<Symbol>> {
        Err(Error::new(
            self.source.clone(),
            Quote::new(start, self.index),
            reason,
        ))
    }

    /// Advance and create an error with the given reason afterwards
    pub fn error_next(&mut self, reason: Reason, start: usize) -> Result<Q<Symbol>> {
        self.advance();
        self.error(reason, start)
    }
}

/// Parsing specific functions
impl Lexer {
    fn skip_line(&mut self) {
        while self.get() != NULL {
            let c = self.get();
            self.advance();
            if matches!(c, b'\n' | b'\r') {
                break;
            }
        }
    }

    fn read_number(&mut self) -> Result<Q<Symbol>> {
        let start_index = self.index;
        while self.get().is_ascii_digit() {
            self.advance();
        }
        self.quote(Symbol::Number, start_index)
    }

    fn read_identifier(&mut self) -> Result<Q<Symbol>> {
        let start_index = self.index;
        while matches!(self.get(), b'_' | b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9') {
            self.advance();
        }
        let slice = &self.source[start_index..self.index];
        if let Some(symbol) = KEYWORDS.get(slice) {
            return self.quote(*symbol, start_index);
        }
        self.quote(Symbol::Identifier, start_index)
    }

    fn read_symbol(&mut self) -> Result<Q<Symbol>> {
        let start_index = self.index;
        let c = self.get();
        if c == NULL {
            return Ok(Q::new(Symbol::Null, start_index, self.index));
        }
        match c {
            b';' => self.quote_next(Symbol::Semicolon, start_index),
            b'=' => self.quote_next(Symbol::Equal, start_index),
            b'.' => self.quote_next(Symbol::Punct, start_index),
            b'(' => self.quote_next(Symbol::LeftParen, start_index),
            b')' => self.quote_next(Symbol::RightParen, start_index),
            b'!' => self.quote_next(Symbol::Not, start_index),
            b'&' => self.quote_next(Symbol::And, start_index),
            b'|' => self.quote_next(Symbol::Or, start_index),
            b'^' => self.quote_next(Symbol::Xor, start_index),
            b'0'..=b'9' => self.read_number(),
            b'_' | b'a'..=b'z' | b'A'..=b'Z' => self.read_identifier(),
            _ => self.error_next(Reason::UnexpectedCharacter, start_index),
        }
    }
}
