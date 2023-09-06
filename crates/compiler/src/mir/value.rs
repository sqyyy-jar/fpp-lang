use std::{fmt::Debug, rc::Rc};

use crate::{
    error::{Error, Reason, Result},
    mir::{MirAction, MirOutputAction},
    util::Quote,
};

use super::{ops::MirOp, writer::MirInstructionWriter, Mir};

#[derive(Clone, Debug)]
pub enum MirValue {
    Bool(MirBool),
    Number(MirNumber),
    BitAddress(MirBitAddress),
    Address(MirAddress),
    VarRef(MirVarRef),
    /// A series of operations expected to return a value
    Ops(Rc<MirOps>),
    Object(Rc<dyn MirObject>),
    Not(Rc<MirNot>),
    And(Rc<MirAnd>),
    Or(Rc<MirOr>),
    Xor(Rc<MirXor>),
}

impl MirValue {
    /// Handler for a write to a variable
    ///
    /// - `&mut Mir`: reference to the MIR
    /// - `quote`: quote of the equals symbol
    /// - `index`: index of the variable to be written to
    /// - `value`: value written
    pub fn write(&self, mir: &mut Mir, quote: Quote, index: usize, value: MirValue) -> Result<()> {
        match self {
            MirValue::BitAddress(addr) => addr.write(mir, quote, index, value),
            MirValue::VarRef(var) => {
                let var_value = mir.variables[var.index].value.clone();
                var_value.write(mir, quote, index, value)?;
                mir.variables[var.index].value = var_value;
                Ok(())
            }
            MirValue::Object(object) => object.write(mir, quote, index, value),
            _ => Err(Error::new(
                mir.source.clone(),
                quote,
                Reason::NoWriteHandler,
            )),
        }
    }
}

pub trait MirObject: Debug {
    /// Handler for a write to a variable
    ///
    /// - `&mut Mir`: reference to the MIR
    /// - `quote`: quote of the equals symbol
    /// - `index`: index of the variable to be written to
    /// - `value`: value written
    fn write(&self, mir: &mut Mir, quote: Quote, _index: usize, _value: MirValue) -> Result<()> {
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
    pub ptr: u16,
    pub bit: u8,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirAddressType {
    Memory8,
    Memory16,
    Memory32,
}

#[derive(Clone, Copy, Debug)]
pub struct MirAddress {
    pub r#type: MirAddressType,
    pub ptr: u16,
}

#[derive(Clone, Copy, Debug)]
pub struct MirVarRef {
    pub index: usize,
}

impl MirBitAddress {
    pub fn write(&self, mir: &mut Mir, quote: Quote, _index: usize, value: MirValue) -> Result<()> {
        if self.r#type != MirBitAddressType::Output {
            return Err(Error::new(
                mir.source.clone(),
                quote,
                Reason::NoWriteHandler,
            ));
        }
        let mut writer = MirInstructionWriter::default();
        writer.write_value(mir, value)?;
        mir.actions.push(MirAction::Output(MirOutputAction {
            address: *self,
            instructions: writer.instructions,
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
