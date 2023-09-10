use std::{fmt::Debug, rc::Rc};

use crate::{
    error::{Error, Reason, Result},
    util::Quote,
};

use super::{
    ops::MirOp,
    writer::{optimizer::optimize, MirInstructionWriter},
    Mir, MirAction, MirOutputAction,
};

#[derive(Clone, Debug)]
pub enum MirValue {
    Bool(MirBool),
    Number(MirNumber),
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
            MirValue::Address(addr) => addr.write(mir, quote, index, value),
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

    pub fn is_bit_readable(&self, mir: &Mir) -> bool {
        match self {
            MirValue::Ops(_) => true,
            MirValue::Address(addr) => addr.is_bit(),
            MirValue::Bool(_) | MirValue::Number(_) | MirValue::Object(_) => false,
            MirValue::VarRef(var) => mir.variables[var.index].value.is_bit_readable(mir),
            MirValue::Not(not) => not.value.is_bit_readable(mir),
            MirValue::And(and) => and.left.is_bit_readable(mir) && and.right.is_bit_readable(mir),
            MirValue::Or(or) => or.left.is_bit_readable(mir) && or.right.is_bit_readable(mir),
            MirValue::Xor(xor) => xor.left.is_bit_readable(mir) && xor.right.is_bit_readable(mir),
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
pub enum MirAddressType {
    PhysicalInput1,
    PhysicalOutput1,
    PhysicalCounter,
    Memory1,
    Memory8,
    Memory16,
    Memory32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MirAddress {
    pub r#type: MirAddressType,
    pub ptr: u32,
}

impl MirAddress {
    pub fn write(&self, mir: &mut Mir, quote: Quote, _index: usize, value: MirValue) -> Result<()> {
        if self.r#type != MirAddressType::PhysicalOutput1 {
            return Err(Error::new(
                mir.source.clone(),
                quote,
                Reason::NoWriteHandler,
            ));
        }
        if !value.is_bit_readable(mir) {
            return Err(Error::new(
                mir.source.clone(),
                quote,
                Reason::ValueNotBitReadable,
            ));
        }
        let mut writer = MirInstructionWriter::default();
        writer.write_value(mir, &value)?;
        optimize(&mut writer);
        mir.actions.push(MirAction::Output(MirOutputAction {
            address: *self,
            instructions: writer.instructions,
        }));
        Ok(())
    }

    pub fn is_physical(self) -> bool {
        matches!(
            self.r#type,
            MirAddressType::PhysicalInput1
                | MirAddressType::PhysicalOutput1
                | MirAddressType::PhysicalCounter
        )
    }

    pub fn is_virtual(self) -> bool {
        !self.is_physical()
    }

    pub fn is_bit(self) -> bool {
        matches!(
            self.r#type,
            MirAddressType::PhysicalInput1
                | MirAddressType::PhysicalOutput1
                | MirAddressType::Memory1
        )
    }
}

impl Debug for MirAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = match self.r#type {
            MirAddressType::PhysicalInput1 => "I",
            MirAddressType::PhysicalOutput1 => "Q",
            MirAddressType::PhysicalCounter => "Z",
            MirAddressType::Memory1 => "M",
            MirAddressType::Memory8 => "MB",
            MirAddressType::Memory16 => "MW",
            MirAddressType::Memory32 => "MD",
        };
        if self.is_bit() && self.is_physical() {
            let ptr = self.ptr & 0xffff;
            let bit = self.ptr >> 16;
            write!(f, "{prefix}{ptr}.{bit}")
        } else {
            write!(f, "{prefix}{}", self.ptr)
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MirVarRef {
    pub index: usize,
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
