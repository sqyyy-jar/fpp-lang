use phf::{phf_map, Map};

pub const KEYWORDS: Map<&[u8], Symbol> = phf_map! {
    b"let" => Symbol::Let,
    b"not" => Symbol::Not,
    b"and" => Symbol::And,
    b"or" => Symbol::Or,
    b"xor" => Symbol::Xor,
    b"true" => Symbol::True,
    b"false" => Symbol::False,
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
    /// `true`
    True,
    /// `false`
    False,
    // --- Literals ---
    Number,
    Identifier,
    // --- Special ---
    Null,
}

impl Symbol {
    pub fn is_unary_op(self) -> bool {
        matches!(self, Symbol::Not)
    }

    pub fn is_binary_op(self) -> bool {
        matches!(self, Symbol::And | Symbol::Or | Symbol::Xor)
    }

    pub fn precedence(self) -> usize {
        match self {
            Symbol::And => 3,
            Symbol::Xor => 2,
            Symbol::Or => 1,
            _ => panic!("Invalid binary op"),
        }
    }
}
