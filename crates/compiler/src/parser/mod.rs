//! This module is responsible for parsing an [Hir] from a [Lexer].

pub mod lexer;
pub mod symbol;

use std::rc::Rc;

use crate::{
    error::{Error, Reason, Result},
    hir::{
        value::{
            HirAnd, HirBitAddress, HirBool, HirCall, HirNot, HirNumber, HirValue, HirValueType,
            HirVarRef,
        },
        Hir, HirCallStatement, HirLetStatement, HirStatement, HirWriteStatement,
    },
    util::{parse_number, Quote, Source, Q},
};

use self::{
    lexer::{Lexer, NULL},
    symbol::Symbol,
};

pub struct Parser {
    source: Rc<Source>,
    lexer: Lexer,
    buffer: Q<Symbol>,
}

/// General parser functions
impl Parser {
    pub fn new(source: Rc<Source>) -> Self {
        Self {
            source: source.clone(),
            lexer: Lexer::new(source),
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
    fn parse_address_prefix(&self, prefix: &str, start: usize, end: usize) -> Result<(u8, usize)> {
        if prefix.len() < 2 {
            return self.error(Reason::InvalidBitAddressSymbol, start, end);
        }
        let char = prefix.chars().next();
        let num = prefix.get(1..);
        if char.is_none() || num.is_none() {
            return self.error(Reason::InvalidBitAddressSymbol, start, end);
        }
        let Ok(x) = num.unwrap().parse() else {
            return self.error(Reason::InvalidBitAddressSymbol, start, end);
        };
        Ok((char.unwrap() as u8, x))
    }

    /// Read raw address (`0.0`)
    fn read_raw_address(&mut self, q_ptr: Quote) -> Result<HirValue> {
        let start = q_ptr.start;
        let ptr = parse_number(&self.source, &q_ptr)?;
        let punct = self.buffer.quote.clone();
        self.advance()?;
        let q_bit = self.expect(Symbol::Number)?;
        let bit = parse_number(&self.source, &q_bit)?;
        let end = q_bit.end;
        if !q_ptr.adjacent(&punct) || !punct.adjacent(&q_bit) || ptr > u16::MAX as usize || bit > 7
        {
            return self.error(Reason::InvalidBitAddressSymbol, start, end);
        }
        let quote = Quote { start, end };
        Ok(HirValue::new(
            quote,
            HirValueType::BitAddress(HirBitAddress {
                char: NULL as u8,
                ptr: ptr as u16,
                bit: bit as u8,
            }),
        ))
    }

    /// Read prefixed address (`E0.0`)
    fn read_prefixed_address(&mut self, prefix: Quote) -> Result<HirValue> {
        let start = prefix.start;
        let punct = self.expect(Symbol::Punct)?;
        let q_bit = self.expect(Symbol::Number)?;
        let bit = parse_number(&self.source, &q_bit)?;
        let end = q_bit.end;
        if !prefix.adjacent(&punct) || !punct.adjacent(&q_bit) || bit > 7 {
            return self.error(Reason::InvalidBitAddressSymbol, start, end);
        }
        let (char, ptr) = self.parse_address_prefix(&self.source.code[&prefix], start, end)?;
        if ptr > u16::MAX as usize {
            return self.error(Reason::InvalidBitAddressSymbol, start, end);
        }
        let ptr = ptr as u16;
        let bit = bit as u8;
        let quote = Quote { start, end };
        Ok(HirValue::new(
            quote,
            HirValueType::BitAddress(HirBitAddress { char, ptr, bit }),
        ))
    }

    /// Read function call (`MB()`)
    fn read_call(&mut self, name: Quote) -> Result<HirValue> {
        let (call, quote) = self.read_call_raw(name)?;
        Ok(HirValue::new(quote, HirValueType::Call(call)))
    }

    /// Read function call (`out(0.0)`)
    fn read_call_raw(&mut self, name: Quote) -> Result<(HirCall, Quote)> {
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
        Ok((HirCall { name, args }, quote))
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
                let value = parse_number(&self.source, &symbol.quote)?;
                Ok(HirValue::new(
                    symbol.quote,
                    HirValueType::Number(HirNumber { value }),
                ))
            }
            Symbol::Identifier => {
                self.advance()?;
                match self.buffer.value {
                    Symbol::Punct => self.read_prefixed_address(symbol.quote),
                    Symbol::LeftParen => self.read_call(symbol.quote),
                    _ => Ok(HirValue::new(symbol.quote, HirValueType::VarRef(HirVarRef))),
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
        Ok(HirStatement::Let(HirLetStatement { quote, name, value }))
    }

    /// Read a [HirWrite]
    fn read_write(&mut self, name: Quote) -> Result<HirStatement> {
        let start = name.start;
        self.expect(Symbol::Equal)?;
        let value = self.read_value()?;
        let end = self.expect(Symbol::Semicolon)?.end;
        let quote = Quote { start, end };
        Ok(HirStatement::Write(HirWriteStatement {
            quote,
            name,
            value,
        }))
    }

    /// Read a statement starting with an [Symbol::Identifier]
    fn read_ident_statement(&mut self) -> Result<HirStatement> {
        let ident = self.expect(Symbol::Identifier)?;
        match self.buffer.value {
            Symbol::Equal => self.read_write(ident),
            Symbol::LeftParen => {
                let (HirCall { name, args }, quote) = self.read_call_raw(ident)?;
                self.expect(Symbol::Semicolon)?;
                Ok(HirStatement::Call(HirCallStatement { quote, name, args }))
            }
            _ => self.error_buffer(Reason::UnexpectedSymbol),
        }
    }

    pub fn parse(&mut self) -> Result<Hir> {
        let mut hir = Hir::new(self.source.clone());
        self.advance()?;
        while self.buffer.value != Symbol::Null {
            hir.statements.push(match self.buffer.value {
                Symbol::Let => self.read_let()?,
                Symbol::Identifier => self.read_ident_statement()?,
                _ => self.error_buffer(Reason::UnexpectedSymbol)?,
            });
        }
        Ok(hir)
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
