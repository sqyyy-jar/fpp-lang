use std::{any::Any, fmt::Debug};

use crate::{
    error::{Error, Reason, Result},
    mir::{MirAction, MirOutputAction},
    util::Quote,
};

use super::Mir;

pub type MirValueBox = Box<dyn MirValue>;

pub trait MirValue: Any + Debug {
    /// Handler for a write to a variable
    ///
    /// - `&mut Mir`: reference to the MIR
    /// - `quote`: quote of the equals symbol
    /// - `index`: index of the variable to be written to
    /// - `value`: value written
    fn write(
        &mut self,
        mir: &mut Mir,
        quote: Quote,
        _index: usize,
        _value: MirValueBox,
    ) -> Result<()> {
        Err(Error::new(
            mir.source.clone(),
            quote,
            Reason::NoWriteHandler,
        ))
    }
}

#[derive(Debug)]
pub enum MirIncompleteValue {
    Not(Box<MirNot>),
    And(Box<MirAnd>),
    Or(Box<MirOr>),
    Xor(Box<MirXor>),
    Variable(MirVariable),
}

impl MirValue for MirIncompleteValue {}

#[derive(Debug)]
pub struct MirBool {
    pub value: bool,
}

impl MirValue for MirBool {}

#[derive(Debug)]
pub struct MirNumber {
    pub value: usize,
}

impl MirValue for MirNumber {}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirBitAddressType {
    Input,
    Output,
    Memory,
}

#[derive(Clone, Copy, Debug)]
pub struct MirBitAddress {
    pub r#type: MirBitAddressType,
    pub x: u16,
    pub y: u8,
}

impl MirValue for MirBitAddress {
    fn write(
        &mut self,
        mir: &mut Mir,
        quote: Quote,
        _index: usize,
        _value: MirValueBox,
    ) -> Result<()> {
        if self.r#type != MirBitAddressType::Output {
            return Err(Error::new(
                mir.source.clone(),
                quote,
                Reason::NoWriteHandler,
            ));
        }
        mir.actions.push(MirAction::Output(MirOutputAction {
            address: *self,
            instructions: vec![],
        }));
        Ok(())
    }
}

#[derive(Debug)]
pub struct MirNot {
    pub value: MirValueBox,
}

#[derive(Debug)]
pub struct MirAnd {
    pub left: MirValueBox,
    pub right: MirValueBox,
}

#[derive(Debug)]
pub struct MirOr {
    pub left: MirValueBox,
    pub right: MirValueBox,
}

#[derive(Debug)]
pub struct MirXor {
    pub left: MirValueBox,
    pub right: MirValueBox,
}

#[derive(Debug)]
pub struct MirVariable {
    pub index: usize,
}
