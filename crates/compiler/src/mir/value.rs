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
    Unit,
    Bool(MirBool),
    Number(MirNumber),
    Address(MirAddress),
    VarRef(MirVarRef),
    /// A series of operations expected to return a bit-value
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
            Self::Address(addr) => addr.write(mir, quote, index, value),
            Self::VarRef(var) => {
                let var_value = mir.variables[var.index].value.clone();
                var_value.write(mir, quote, index, value)?;
                mir.variables[var.index].value = var_value;
                Ok(())
            }
            Self::Object(object) => object.write(mir, quote, index, value),
            _ => Err(Error::new(
                mir.source.clone(),
                quote,
                Reason::NoWriteHandler,
            )),
        }
    }

    pub fn is_bit_readable(&self, mir: &Mir) -> bool {
        match self {
            Self::Ops(_) => true,
            Self::Address(addr) => addr.is_bit_address(),
            Self::Unit | Self::Bool(_) | Self::Number(_) | Self::Object(_) => false,
            Self::VarRef(var) => mir.variables[var.index].value.is_bit_readable(mir),
            Self::Not(not) => not.value.is_bit_readable(mir),
            Self::And(and) => and.left.is_bit_readable(mir) && and.right.is_bit_readable(mir),
            Self::Or(or) => or.left.is_bit_readable(mir) && or.right.is_bit_readable(mir),
            Self::Xor(xor) => xor.left.is_bit_readable(mir) && xor.right.is_bit_readable(mir),
        }
    }

    pub fn is_unit(&self) -> bool {
        matches!(self, Self::Unit)
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
    pub ptr: u16,
    pub bit: u8,
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

    /// Checks if the address points to a `BIT`
    pub fn is_bit_address(self) -> bool {
        matches!(
            self.r#type,
            MirAddressType::PhysicalInput1
                | MirAddressType::PhysicalOutput1
                | MirAddressType::Memory1
        )
    }

    /// Checks if the address points to a `BYTE`, `WORD` or `DWORD`
    pub fn is_any_byte_address(self) -> bool {
        matches!(
            self.r#type,
            MirAddressType::Memory8 | MirAddressType::Memory16 | MirAddressType::Memory32
        )
    }
}

impl Debug for MirAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = match self.r#type {
            MirAddressType::PhysicalInput1 => "I",
            MirAddressType::PhysicalOutput1 => "Q",
            MirAddressType::PhysicalCounter => "C",
            MirAddressType::Memory1 => "M",
            MirAddressType::Memory8 => "MB",
            MirAddressType::Memory16 => "MW",
            MirAddressType::Memory32 => "MD",
        };
        if self.is_bit_address() && self.is_physical() {
            write!(f, "{prefix}{}.{}", self.ptr, self.bit)
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
