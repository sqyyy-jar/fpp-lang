use crate::error::Result;

use super::{value::MirValue, Mir, MirInstruction};

#[derive(Default)]
pub struct MirInstructionWriter {
    pub instructions: Vec<MirInstruction>,
}

impl MirInstructionWriter {
    pub fn write_value(&mut self, _mir: &mut Mir, _value: MirValue) -> Result<()> {
        todo!()
    }
}
