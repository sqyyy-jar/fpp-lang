use std::{fmt::Debug, rc::Rc};

use crate::{
    error::{Error, Reason, Result},
    mir::{MirAction, MirOutputAction},
    util::Quote,
};

use super::{ops::MirOp, Mir};

#[derive(Clone, Debug)]
pub enum MirValue {
    Bool(MirBool),
    Number(MirNumber),
    BitAddress(MirBitAddress),
    Variable(MirVariable),
    /// A series of operations expected to return a value
    Ops(Rc<MirOps>),
    Object(Rc<dyn MirObject>),
    Not(Rc<MirNot>),
    And(Rc<MirAnd>),
    Or(Rc<MirOr>),
    Xor(Rc<MirXor>),
}

pub trait MirObject: Debug {
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
        _value: MirValue,
    ) -> Result<()> {
        Err(Error::new(
            mir.source.clone(),
            quote,
            Reason::NoWriteHandler,
        ))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MirBool {
    pub value: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct MirNumber {
    pub value: usize,
}

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

#[derive(Clone, Copy, Debug)]
pub struct MirVariable {
    pub index: usize,
}

// todo
impl MirObject for MirBitAddress {
    fn write(
        &mut self,
        mir: &mut Mir,
        quote: Quote,
        _index: usize,
        _value: MirValue,
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
pub struct MirOps {
    pub ops: Vec<MirOp>,
}

#[derive(Debug)]
pub struct MirNot {
    pub value: MirValue,
}

#[derive(Debug)]
pub struct MirAnd {
    pub left: MirValue,
    pub right: MirValue,
}

#[derive(Debug)]
pub struct MirOr {
    pub left: MirValue,
    pub right: MirValue,
}

#[derive(Debug)]
pub struct MirXor {
    pub left: MirValue,
    pub right: MirValue,
}