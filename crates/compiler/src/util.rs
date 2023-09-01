use std::ops::Index;

/// Represents a value associated with a range of text
#[derive(Clone)]
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
}

impl Index<&Quote> for [u8] {
    type Output = [u8];

    fn index(&self, index: &Quote) -> &Self::Output {
        &self[index.start..index.end]
    }
}
