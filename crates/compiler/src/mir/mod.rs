use std::rc::Rc;

use phf::{phf_map, Map};

use crate::{error::Result, util::Quote};

use self::value::{MirBitAddress, MirValue};

pub mod builtin;
pub mod ops;
pub mod value;

pub const BUILTIN_FUNCTIONS: Map<&[u8], MirFunction> = phf_map! {
    b"rs" => builtin::flipflops::builtin_rs,
};

/// MIR Function
///
/// - `&mut Mir`: reference to the MIR
/// - `quote`: quote of the call
/// - `args`: arguments of the call
pub type MirFunction = fn(&mut Mir, quote: Quote, args: &[MirValue]) -> Result<MirValue>;

pub struct Mir {
    pub source: Rc<[u8]>,
    pub memory: MirMemory,
    pub variables: Vec<MirVariable>,
    pub actions: Vec<MirAction>,
}

impl Mir {
    pub fn new(source: Rc<[u8]>) -> Self {
        Self {
            source,
            memory: MirMemory::default(),
            variables: Vec::new(),
            actions: Vec::new(),
        }
    }
}

#[derive(Default)]
pub struct MirMemory {
    pub byte_index: usize,
    pub bit_index: u8,
}

impl MirMemory {
    pub fn alloc_bit(&mut self) -> Option<(u16, u8)> {
        if self.bit_index >= 7 {
            self.bit_index = 0;
            self.byte_index += 1;
        }
        if self.byte_index > u16::MAX as usize {
            return None;
        }
        let byte = self.byte_index as u16;
        let bit = self.bit_index;
        self.bit_index += 1;
        Some((byte, bit))
    }
}

pub struct MirVariable {
    pub name: Quote,
    pub value: MirValue,
}

pub enum MirAction {
    Output(MirOutputAction),
}

pub struct MirOutputAction {
    pub address: MirBitAddress,
    pub instructions: Vec<MirInstruction>,
}

pub enum MirInstruction {
    /// `SET`
    Set,
    /// `CLR`
    Clear,
    /// `N`
    Not,
    /// `U op`
    And { op: MirBitAddress },
    /// `O op`
    Or { op: MirBitAddress },
    /// `X op`
    Xor { op: MirBitAddress },
    /// `= dst`
    WriteBit { dst: MirBitAddress },
    /// `S dst`
    SetBit { dst: MirBitAddress },
    /// `R dst`
    ResetBit { dst: MirBitAddress },
}
