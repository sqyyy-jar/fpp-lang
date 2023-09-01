use std::rc::Rc;

use crate::{
    error::{Error, Reason, Result},
    hir::{
        value::{HirAnd, HirBool, HirCall, HirInput, HirNot, HirNumber, HirValue, HirVariable},
        Hir, HirLet, HirStatement, HirWrite,
    },
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
        self.error_buffer(Reason::UnexpectedSymbol)
    }

    fn error<T>(&self, reason: Reason, start: usize, end: usize) -> Result<T> {
        Err(Error::new(
            self.source.clone(),
            Quote { start, end },
            reason,
        ))
    }

    fn error_buffer<T>(&self, reason: Reason) -> Result<T> {
        Err(Error::new(
            self.source.clone(),
            self.buffer.quote.clone(),
            reason,
        ))
    }
}

/// Parsing specific functions
impl Parser {
    /// Read input symbol (`E0.0`)
    fn read_input_symbol(&mut self) -> Result<HirValue> {
        let ident = self.expect(Symbol::Identifier)?;
        let start = ident.start;
        let slice = &self.source[&ident];
        if slice != [self.opts.input_char] {
            return self.error_buffer(Reason::InvalidInputSymbol);
        }
        let x = self.expect(Symbol::Number)?;
        let punct = self.expect(Symbol::Punct)?;
        let y = self.expect(Symbol::Number)?;
        let end = y.end;
        if !ident.adjacent(&x) || !x.adjacent(&punct) || !punct.adjacent(&y) {
            return self.error(Reason::InvalidInputSymbol, start, end);
        }
        let quote = Quote { start, end };
        Ok(HirValue::Input(HirInput { quote, x, y }))
    }

    /// Read output symbol (`A0.0`)
    fn read_output_symbol(&mut self) -> Result<HirValue> {
        let ident = self.expect(Symbol::Identifier)?;
        let start = ident.start;
        let slice = &self.source[&ident];
        if slice != [self.opts.output_char] {
            return self.error_buffer(Reason::InvalidOutputSymbol);
        }
        let x = self.expect(Symbol::Number)?;
        let punct = self.expect(Symbol::Punct)?;
        let y = self.expect(Symbol::Number)?;
        let end = y.end;
        if !ident.adjacent(&x) || !x.adjacent(&punct) || !punct.adjacent(&y) {
            return self.error(Reason::InvalidOutputSymbol, start, end);
        }
        let quote = Quote { start, end };
        Ok(HirValue::Input(HirInput { quote, x, y }))
    }

    /// Read function call (`out(0.0)`)
    fn read_call(&mut self, name: Quote) -> Result<HirValue> {
        let start = name.start;
        self.expect(Symbol::LeftParen)?;
        let mut args = Vec::with_capacity(1);
        while self.buffer.value != Symbol::RightParen {
            args.push(self.read_value()?);
            if self.buffer.value != Symbol::Comma {
                break;
            }
            self.advance()?;
        }
        let end = self.expect(Symbol::RightParen)?.end;
        let quote = Quote { start, end };
        Ok(HirValue::Call(HirCall { quote, name, args }))
    }

    /// Read unary operation (`not value`)
    fn read_unary_value(&mut self) -> Result<HirValue> {
        let symbol = self.get();
        if symbol.value.is_unary_op() {
            self.advance()?;
            return Ok(apply_unary(symbol, self.read_unary_value()?));
        }
        match symbol.value {
            Symbol::LeftParen => {
                self.advance()?;
                let value = self.read_value()?;
                self.expect(Symbol::RightParen)?;
                // todo: incorrect quote
                Ok(value)
            }
            Symbol::True | Symbol::False => {
                self.advance()?;
                Ok(HirValue::Bool(HirBool {
                    quote: symbol.quote,
                    value: symbol.value == Symbol::True,
                }))
            }
            Symbol::Number => {
                self.advance()?;
                Ok(HirValue::Number(HirNumber {
                    quote: symbol.quote,
                }))
            }
            Symbol::Identifier => {
                let slice = &self.source[&symbol.quote];
                if self.opts.input_symbols && slice == [self.opts.input_char] {
                    return self.read_input_symbol();
                } else if self.opts.output_symbols && slice == [self.opts.output_char] {
                    return self.read_output_symbol();
                }
                self.advance()?;
                if self.buffer.value == Symbol::LeftParen {
                    return self.read_call(symbol.quote);
                }
                Ok(HirValue::Variable(HirVariable {
                    quote: symbol.quote,
                }))
            }
            _ => self.error_buffer(Reason::InvalidUnaryOperation),
        }
    }

    /// Read binary operation (`x and y`)
    fn read_binary_value(&mut self, left: HirValue) -> Result<HirValue> {
        let op = self.get();
        self.advance()?;
        let right = self.read_unary_value()?;
        if !self.buffer.value.is_binary_op() {
            return Ok(apply_binary(op, left, right));
        }
        if op.value.precedence() < self.buffer.value.precedence() {
            return Ok(apply_binary(op, left, self.read_binary_value(right)?));
        }
        self.read_binary_value(apply_binary(op, left, right))
    }

    /// Read a [HirValue]
    fn read_value(&mut self) -> Result<HirValue> {
        let left = self.read_unary_value()?;
        if !self.buffer.value.is_binary_op() {
            return Ok(left);
        }
        self.read_binary_value(left)
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
                _ => self.error_buffer(Reason::UnexpectedSymbol)?,
            });
        }
        Ok(hir)
    }
}

pub struct ParserOptions {
    pub input_symbols: bool,
    pub output_symbols: bool,
    pub input_char: u8,
    pub output_char: u8,
}

impl Default for ParserOptions {
    fn default() -> Self {
        Self {
            input_symbols: true,
            output_symbols: true,
            input_char: b'E',
            output_char: b'A',
        }
    }
}

fn apply_unary(op: Q<Symbol>, value: HirValue) -> HirValue {
    let quote = Quote::new(op.quote.start, value.quote().end);
    match op.value {
        Symbol::Not => HirValue::Not(Box::new(HirNot { quote, value })),
        _ => panic!("Invalid unary op"),
    }
}

fn apply_binary(op: Q<Symbol>, left: HirValue, right: HirValue) -> HirValue {
    let quote = Quote::new(left.quote().start, right.quote().end);
    match op.value {
        Symbol::And => HirValue::And(Box::new(HirAnd { quote, left, right })),
        Symbol::Or => HirValue::And(Box::new(HirAnd { quote, left, right })),
        Symbol::Xor => HirValue::And(Box::new(HirAnd { quote, left, right })),
        _ => panic!("Invalid binary op"),
    }
}
