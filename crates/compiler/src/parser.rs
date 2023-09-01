use std::rc::Rc;

use crate::{
    error::{Error, Reason, Result},
    hir::{value::HirValue, Hir, HirLet, HirStatement, HirWrite},
    lexer::Lexer,
    symbol::Symbol,
    util::{Quote, Q},
};

pub struct Parser {
    source: Rc<[u8]>,
    lexer: Lexer,
    opts: ParserOptions,
    buffer: Q<Symbol>,
}

/// General parser functions
impl Parser {
    pub fn new(source: Rc<[u8]>, opts: ParserOptions) -> Self {
        Self {
            source: source.clone(),
            lexer: Lexer::new(source),
            opts,
            buffer: Q::new(Symbol::Null, 0, 0),
        }
    }

    /// Get the current symbol
    fn get(&self) -> Q<Symbol> {
        self.buffer.clone()
    }

    /// Read the next symbol
    fn advance(&mut self) -> Result<()> {
        self.buffer = self.lexer.read_symbol()?;
        Ok(())
    }

    fn expect(&mut self, value: Symbol) -> Result<Quote> {
        if self.buffer.value == value {
            return Ok(self.buffer.quote.clone());
        }
        self.error(Reason::UnexpectedSymbol)
    }

    fn error<T>(&self, reason: Reason) -> Result<T> {
        Err(Error::new(
            self.source.clone(),
            self.buffer.quote.clone(),
            reason,
        ))
    }
}

/// Parsing specific functions
impl Parser {
    /// Read a [HirValue]
    fn read_value(&mut self) -> Result<HirValue> {
        todo!()
    }

    /// Read a [HirLet]
    fn read_let(&mut self) -> Result<HirStatement> {
        let start = self.expect(Symbol::Let)?.start;
        let name = self.expect(Symbol::Identifier)?;
        self.expect(Symbol::Equal)?;
        let value = self.read_value()?;
        let end = self.expect(Symbol::Semicolon)?.end;
        let quote = Quote { start, end };
        Ok(HirStatement::Let(HirLet { quote, name, value }))
    }

    /// Read a [HirWrite]
    fn read_write(&mut self) -> Result<HirStatement> {
        let start = self.buffer.quote.start;
        let name = self.expect(Symbol::Identifier)?;
        self.expect(Symbol::Equal)?;
        let value = self.read_value()?;
        let end = self.expect(Symbol::Semicolon)?.end;
        let quote = Quote { start, end };
        Ok(HirStatement::Write(HirWrite { quote, name, value }))
    }

    pub fn parse(&mut self) -> Result<Hir> {
        let mut hir = Hir::default();
        self.advance()?;
        while self.buffer.value != Symbol::Null {
            hir.statements.push(match self.buffer.value {
                Symbol::Let => self.read_let()?,
                Symbol::Identifier => self.read_write()?,
                _ => self.error(Reason::UnexpectedSymbol)?,
            });
        }
        Ok(hir)
    }
}

pub struct ParserOptions {
    pub input_symbols: bool,
    pub output_symbols: bool,
}

impl Default for ParserOptions {
    fn default() -> Self {
        Self {
            input_symbols: true,
            output_symbols: true,
        }
    }
}
