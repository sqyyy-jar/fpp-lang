use std::{fmt::Debug, rc::Rc};

use crate::util::Quote;

pub type Result<T> = std::result::Result<T, Error>;

pub struct Error {
    source: Rc<str>,
    quote: Quote,
    reason: Reason,
}

impl Error {
    pub fn new(source: Rc<str>, quote: Quote, reason: Reason) -> Self {
        Self {
            source,
            quote,
            reason,
        }
    }
}

/// **NOTE: This Debug implementation will be removed in the future**
impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Error")
            .field("reason", &self.reason)
            .field("source", &&self.source[&self.quote])
            .finish()
    }
}

#[derive(Debug)]
pub enum Reason {
    // Lexer
    UnexpectedCharacter,
    InvalidNumber,
    // Parser
    UnexpectedSymbol,
    InvalidBitAddressSymbol,
    InvalidUnaryOperation,
    // Mir
    NoWriteHandler,
    InvalidArgsCount,
    InvalidArgType,
    UnknownVariable,
    UnknownFunction,
    UnknownBitAddressType,
    ValueNotBitReadable,
}
