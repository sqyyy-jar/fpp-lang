//! This module is responsible for parsing source code into a stream of [Symbol]s.

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
    fn get(&self) -> u8 {
        self.source.get(self.index).copied().unwrap_or(NULL)
    }

    /// Get the next char
    fn peek(&self) -> u8 {
        self.source.get(self.index + 1).copied().unwrap_or(NULL)
    }

    /// Increment the current index
    fn advance(&mut self) {
        self.index += 1;
    }

    /// Quote the symbol
    fn quote(&mut self, value: Symbol, start: usize) -> Result<Q<Symbol>> {
        Ok(Q::new(value, start, self.index))
    }

    /// Advance and quote the symbol afterwards
    fn quote_next(&mut self, value: Symbol, start: usize) -> Result<Q<Symbol>> {
        self.advance();
        self.quote(value, start)
    }

    /// Create an error with the given reason
    fn error(&self, reason: Reason, start: usize) -> Result<Q<Symbol>> {
        Err(Error::new(
            self.source.clone(),
            Quote::new(start, self.index),
            reason,
        ))
    }

    /// Advance and create an error with the given reason afterwards
    fn error_next(&mut self, reason: Reason, start: usize) -> Result<Q<Symbol>> {
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

    fn skip_whitespace(&mut self) {
        while self.get().is_ascii_whitespace() {
            self.advance();
        }
    }

    fn read_number(&mut self) -> Result<Q<Symbol>> {
        let start_index = self.index;
        while self.get().is_ascii_digit() {
            self.advance();
        }
        let slice = &self.source[start_index..self.index];
        if unsafe { std::str::from_utf8_unchecked(slice) }
            .parse::<usize>()
            .is_err()
        {
            return self.error(Reason::InvalidNumber, start_index);
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

    pub fn read_symbol(&mut self) -> Result<Q<Symbol>> {
        loop {
            self.skip_whitespace();
            let start_index = self.index;
            return match self.get() {
                NULL => Ok(Q::new(Symbol::Null, start_index, self.index)),
                b';' => self.quote_next(Symbol::Semicolon, start_index),
                b'=' => self.quote_next(Symbol::Equal, start_index),
                b'.' => self.quote_next(Symbol::Punct, start_index),
                b',' => self.quote_next(Symbol::Comma, start_index),
                b'(' => self.quote_next(Symbol::LeftParen, start_index),
                b')' => self.quote_next(Symbol::RightParen, start_index),
                b'!' => self.quote_next(Symbol::Not, start_index),
                b'&' => self.quote_next(Symbol::And, start_index),
                b'|' => self.quote_next(Symbol::Or, start_index),
                b'^' => self.quote_next(Symbol::Xor, start_index),
                b'0'..=b'9' => self.read_number(),
                b'_' | b'a'..=b'z' | b'A'..=b'Z' => self.read_identifier(),
                b'#' => {
                    self.skip_line();
                    continue;
                }
                b'/' => {
                    if self.peek() != b'/' {
                        return self.error_next(Reason::UnexpectedCharacter, start_index);
                    }
                    self.skip_line();
                    continue;
                }
                _ => self.error_next(Reason::UnexpectedCharacter, start_index),
            };
        }
    }
}
