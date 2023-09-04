//! This module is responsible for parsing an [Hir] from a [Lexer].

use std::rc::Rc;

use crate::{
    error::{Error, Reason, Result},
    hir::{
        value::{
            HirAddress, HirAnd, HirBool, HirCall, HirInput, HirNot, HirNumber, HirOutput, HirValue,
            HirValueType, HirVariable,
        },
        Hir, HirLet, HirStatement, HirWrite,
    },
    lexer::{Lexer, NULL},
    symbol::Symbol,
    util::{parse_number, Quote, Q},
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
            let quote = self.buffer.quote.clone();
            self.advance()?;
            return Ok(quote);
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
    fn parse_address_prefix(&self, prefix: &[u8], start: usize, end: usize) -> Result<(u8, usize)> {
        if prefix.len() < 2 {
            return self.error(Reason::InvalidAddressSymbol, start, end);
        }
        let char = prefix[0];
        let Ok(slice_x) = std::str::from_utf8(&prefix[1..]) else {
            return self.error(Reason::InvalidAddressSymbol, start, end);
        };
        let Ok(x) = slice_x.parse() else {
            return self.error(Reason::InvalidAddressSymbol, start, end);
        };
        Ok((char, x))
    }

    /// Read raw address (`0.0`)
    fn read_raw_address(&mut self, q_x: Quote) -> Result<HirValue> {
        let start = q_x.start;
        let x = parse_number(&self.source[&q_x]);
        let punct = self.buffer.quote.clone();
        self.advance()?;
        let q_y = self.expect(Symbol::Number)?;
        let y = parse_number(&self.source[&q_y]);
        let end = q_y.end;
        if !q_x.adjacent(&punct) || !punct.adjacent(&q_y) {
            return self.error(Reason::InvalidAddressSymbol, start, end);
        }
        let quote = Quote { start, end };
        Ok(HirValue::new(
            quote,
            HirValueType::Address(HirAddress { char: NULL, x, y }),
        ))
    }

    /// Read prefixed address (`E0.0`)
    fn read_prefixed_address(&mut self, prefix: Quote) -> Result<HirValue> {
        let start = prefix.start;
        let punct = self.expect(Symbol::Punct)?;
        let q_y = self.expect(Symbol::Number)?;
        let y = parse_number(&self.source[&q_y]);
        let end = q_y.end;
        if !prefix.adjacent(&punct) || !punct.adjacent(&q_y) {
            return self.error(Reason::InvalidAddressSymbol, start, end);
        }
        let (char, x) = self.parse_address_prefix(&self.source[&prefix], start, end)?;
        let quote = Quote { start, end };
        if self.opts.input_char == char {
            return Ok(HirValue::new(quote, HirValueType::Input(HirInput { x, y })));
        }
        if self.opts.output_char == char {
            return Ok(HirValue::new(
                quote,
                HirValueType::Output(HirOutput { x, y }),
            ));
        }
        Ok(HirValue::new(
            quote,
            HirValueType::Address(HirAddress { char, x, y }),
        ))
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
        Ok(HirValue::new(
            quote,
            HirValueType::Call(HirCall { name, args }),
        ))
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
                Ok(HirValue::new(
                    symbol.quote,
                    HirValueType::Bool(HirBool {
                        value: symbol.value == Symbol::True,
                    }),
                ))
            }
            Symbol::Number => {
                self.advance()?;
                if self.buffer.value == Symbol::Punct {
                    return self.read_raw_address(symbol.quote);
                }
                Ok(HirValue::new(symbol.quote, HirValueType::Number(HirNumber)))
            }
            Symbol::Identifier => {
                self.advance()?;
                match self.buffer.value {
                    Symbol::Punct => self.read_prefixed_address(symbol.quote),
                    Symbol::LeftParen => self.read_call(symbol.quote),
                    _ => Ok(HirValue::new(
                        symbol.quote,
                        HirValueType::Variable(HirVariable),
                    )),
                }
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
        let mut hir = Hir::new(self.source.clone());
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
    pub input_char: u8,
    pub output_char: u8,
}

impl Default for ParserOptions {
    fn default() -> Self {
        Self {
            input_char: b'E',
            output_char: b'A',
        }
    }
}

fn apply_unary(op: Q<Symbol>, value: HirValue) -> HirValue {
    let quote = Quote::new(op.quote.start, value.quote.end);
    match op.value {
        Symbol::Not => HirValue::new(quote, HirValueType::Not(Box::new(HirNot { value }))),
        _ => panic!("Invalid unary op"),
    }
}

fn apply_binary(op: Q<Symbol>, left: HirValue, right: HirValue) -> HirValue {
    let quote = Quote::new(left.quote.start, right.quote.end);
    match op.value {
        Symbol::And => HirValue::new(quote, HirValueType::And(Box::new(HirAnd { left, right }))),
        Symbol::Or => HirValue::new(quote, HirValueType::And(Box::new(HirAnd { left, right }))),
        Symbol::Xor => HirValue::new(quote, HirValueType::And(Box::new(HirAnd { left, right }))),
        _ => panic!("Invalid binary op"),
    }
}
