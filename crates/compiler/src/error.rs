use std::rc::Rc;

use crate::util::Quote;

pub type Result<T> = std::result::Result<T, Error>;

pub struct Error {
    source: Rc<[u8]>,
    quote: Quote,
    reason: Reason,
}

impl Error {
    pub fn new(source: Rc<[u8]>, quote: Quote, reason: Reason) -> Self {
        Self {
            source,
            quote,
            reason,
        }
    }
}

pub enum Reason {
    UnexpectedCharacter,
}
