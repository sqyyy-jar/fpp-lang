pub mod optimizer;

use std::rc::Rc;

use crate::{error::Result, mir::ops::MirOp};

use super::{
    value::{MirAddress, MirAnd, MirNot, MirOps, MirOr, MirValue, MirVarRef, MirXor},
    Mir, MirInstruction,
};

#[derive(Default)]
pub struct MirInstructionWriter {
    pub instructions: Vec<MirInstruction>,
}

impl MirInstructionWriter {
    fn write_addr(&mut self, addr: MirAddress) -> Result<()> {
        assert!(addr.is_bit_address());
        self.instructions.push(MirInstruction::And { addr });
        Ok(())
    }

    fn write_var_ref(&mut self, mir: &mut Mir, var_ref: MirVarRef) -> Result<()> {
        let value = mir.variables[var_ref.index].value.clone();
        self.write_value(mir, &value)
    }

    fn write_ops(&mut self, mir: &mut Mir, ops: &Rc<MirOps>) -> Result<()> {
        for op in &ops.ops {
            match op {
                MirOp::SetBit { cond, addr } => {
                    self.write_value(mir, cond)?;
                    self.instructions
                        .push(MirInstruction::SetBit { addr: *addr })
                }
                MirOp::ResetBit { cond, addr } => {
                    self.write_value(mir, cond)?;
                    self.instructions
                        .push(MirInstruction::ResetBit { addr: *addr })
                }
                MirOp::And { addr } => self.instructions.push(MirInstruction::And { addr: *addr }),
            }
        }
        Ok(())
    }

    fn write_not(&mut self, mir: &mut Mir, not: &Rc<MirNot>) -> Result<()> {
        self.write_value(mir, &not.value)?;
        self.instructions.push(MirInstruction::Not);
        Ok(())
    }

    fn write_and(&mut self, mir: &mut Mir, and: &Rc<MirAnd>) -> Result<()> {
        self.write_value(mir, &and.left)?;
        self.instructions.push(MirInstruction::AndStart);
        self.write_value(mir, &and.right)?;
        self.instructions.push(MirInstruction::End);
        Ok(())
    }

    fn write_or(&mut self, mir: &mut Mir, or: &Rc<MirOr>) -> Result<()> {
        self.write_value(mir, &or.left)?;
        self.instructions.push(MirInstruction::OrStart);
        self.write_value(mir, &or.right)?;
        self.instructions.push(MirInstruction::End);
        Ok(())
    }

    fn write_xor(&mut self, mir: &mut Mir, xor: &Rc<MirXor>) -> Result<()> {
        self.write_value(mir, &xor.left)?;
        self.instructions.push(MirInstruction::XorStart);
        self.write_value(mir, &xor.right)?;
        self.instructions.push(MirInstruction::End);
        Ok(())
    }

    pub fn write_value(&mut self, mir: &mut Mir, value: &MirValue) -> Result<()> {
        match value {
            MirValue::Unit => todo!(),
            MirValue::Bool(_) => todo!(),
            MirValue::Number(_) => todo!(),
            MirValue::Address(addr) => self.write_addr(*addr),
            MirValue::VarRef(var_ref) => self.write_var_ref(mir, *var_ref),
            MirValue::Ops(ops) => self.write_ops(mir, ops),
            MirValue::Object(_) => todo!(),
            MirValue::Not(not) => self.write_not(mir, not),
            MirValue::And(and) => self.write_and(mir, and),
            MirValue::Or(or) => self.write_or(mir, or),
            MirValue::Xor(xor) => self.write_xor(mir, xor),
        }
    }
}
