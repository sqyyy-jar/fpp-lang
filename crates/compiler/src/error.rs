use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

use messages::message::{Message, MessageContent};

use crate::util::{Quote, Source};

pub type Result<T> = std::result::Result<T, Error>;

pub struct Error {
    source: Rc<Source>,
    quote: Quote,
    reason: Reason,
}

impl Error {
    pub fn new(source: Rc<Source>, quote: Quote, reason: Reason) -> Self {
        Self {
            source,
            quote,
            reason,
        }
    }

    pub fn message(&self) -> &str {
        match self.reason {
            Reason::UnexpectedCharacter => "this character was not expected",
            Reason::InvalidNumber => "this number is not valid",
            Reason::UnexpectedSymbol => "this symbol was not expected",
            Reason::InvalidBitAddressSymbol => "this bit-address is invalid",
            Reason::InvalidUnaryOperation => "this unary operation is invalid",
            Reason::NoWriteHandler => "there is no write handler available for this variable",
            Reason::InvalidArgsCount => "the amount of args does not match the function signature",
            Reason::InvalidArgType => "the arguments do not match the function signature",
            Reason::UnknownVariable => "this variable does not exist",
            Reason::UnknownFunction => "this function does not exist",
            Reason::UnknownBitAddressType => "this bit-address type is invalid",
            Reason::ValueNotBitReadable => "this value is not readable",
        }
    }

    pub fn content(&self) -> Option<MessageContent> {
        MessageContent::parse(
            &self.source.file,
            &self.source.code,
            self.quote.start,
            self.quote.end,
        )
    }

    pub fn to_message(&self) -> Option<Message> {
        let content = self.content()?;
        Some(Message::error(content, self.message()))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Some(message) = self.to_message() else {
            return Ok(());
        };
        message.fmt(f)
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
