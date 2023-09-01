use phf::{phf_map, Map};

pub const KEYWORDS: Map<&[u8], Symbol> = phf_map! {
    b"let" => Symbol::Let,
    b"not" => Symbol::Not,
    b"and" => Symbol::And,
    b"or" => Symbol::Or,
    b"xor" => Symbol::Xor,
};

/// A part of the parsed source
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Symbol {
    // --- Punctuation ---
    /// `;`
    Semicolon,
    /// `=`
    Equal,
    /// `.`
    Punct,
    /// `,`
    Comma,
    /// `(`
    LeftParen,
    /// `)`
    RightParen,
    // --- Keywords ---
    /// `let`
    Let,
    /// `not`, `!`
    Not,
    /// `and`, `&`
    And,
    /// `or`, `|`
    Or,
    /// `xor`, `^`
    Xor,
    // --- Literals ---
    Number,
    Identifier,
    // --- Special ---
    Null,
}
