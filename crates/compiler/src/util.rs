use std::{fmt::Debug, ops::Index, rc::Rc};

use crate::error::{Error, Reason, Result};

/// Represents a value associated with a range of text
#[derive(Clone, Debug)]
pub struct Q<T> {
    pub value: T,
    pub quote: Quote,
}

impl<T> Q<T> {
    pub fn new(value: T, start: usize, end: usize) -> Self {
        Self {
            value,
            quote: Quote::new(start, end),
        }
    }
}

/// Represents a range of text
#[derive(Clone)]
pub struct Quote {
    pub start: usize,
    pub end: usize,
}

impl Quote {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Checks if two quotes are adjacent
    pub fn adjacent(&self, other: &Self) -> bool {
        self.end == other.start || self.start == other.end
    }
}

impl Debug for Quote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}..{}", self.start, self.end))
    }
}

impl Index<&Quote> for [u8] {
    type Output = [u8];

    fn index(&self, index: &Quote) -> &Self::Output {
        &self[index.start..index.end]
    }
}

pub fn parse_number(source: &Rc<[u8]>, quote: &Quote) -> Result<usize> {
    std::str::from_utf8(&source[quote])
        .map_err(|_| Error::new(source.clone(), quote.clone(), Reason::InvalidNumber))?
        .parse()
        .map_err(|_| Error::new(source.clone(), quote.clone(), Reason::InvalidNumber))
}
