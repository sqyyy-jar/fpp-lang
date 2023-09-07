use std::{fmt::Debug, rc::Rc};

use phf::{phf_map, Map};

use crate::{error::Result, util::Quote};

use self::value::{MirBitAddress, MirValue};

pub mod builtin;
pub mod ops;
pub mod transformer;
pub mod value;
pub mod writer;

pub const BUILTIN_FUNCTIONS: Map<&[u8], MirFunction> = phf_map! {
    b"MB" => builtin::memory::builtin_mb,
    b"MW" => builtin::memory::builtin_mw,
    b"MD" => builtin::memory::builtin_md,
    b"rs" => builtin::flipflops::builtin_rs,
    b"sr" => builtin::flipflops::builtin_sr,
};

/// MIR Function
///
/// - `&mut Mir`: reference to the MIR
/// - `quote`: quote of the call
/// - `args`: arguments of the call
pub type MirFunction = fn(&mut Mir, quote: Quote, args: &[MirValue]) -> Result<MirValue>;

#[derive(Debug)]
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

    pub fn find_var(&self, name: &[u8]) -> Option<usize> {
        for (i, var) in self.variables.iter().enumerate() {
            if name == &self.source[&var.name] {
                return Some(i);
            }
        }
        None
    }
}

#[derive(Debug, Default)]
pub struct MirMemory {
    pub byte_index: usize,
    pub bit_index: u8,
}

impl MirMemory {
    pub fn alloc_u1(&mut self) -> Option<(u16, u8)> {
        if self.bit_index >= 7 {
            self.bit_index = 0;
            self.byte_index += 1;
        }
        let ptr = self.byte_index;
        if ptr > 0xffff {
            return None;
        }
        let bit = self.bit_index;
        self.bit_index += 1;
        Some((ptr as u16, bit))
    }

    pub fn alloc_u8(&mut self) -> Option<u16> {
        if self.bit_index > 0 {
            self.bit_index = 0;
            self.byte_index += 1;
        }
        let ptr = self.byte_index;
        if ptr > 0xffff {
            return None;
        }
        self.byte_index += 1;
        Some(ptr as u16)
    }

    pub fn alloc_u16(&mut self) -> Option<u16> {
        if self.bit_index > 0 {
            self.bit_index = 0;
            self.byte_index += 1;
        }
        let ptr = self.byte_index;
        if ptr > 0xfffe {
            return None;
        }
        self.byte_index += 2;
        Some(ptr as u16)
    }

    pub fn alloc_u32(&mut self) -> Option<u16> {
        if self.bit_index > 0 {
            self.bit_index = 0;
            self.byte_index += 1;
        }
        let ptr = self.byte_index;
        if ptr > 0xfffc {
            return None;
        }
        self.byte_index += 4;
        Some(ptr as u16)
    }
}

#[derive(Debug)]
pub struct MirVariable {
    pub name: Quote,
    pub value: MirValue,
}

#[derive(Debug)]
pub enum MirAction {
    Output(MirOutputAction),
}

#[derive(Debug)]
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
    And { addr: MirBitAddress },
    /// `O op`
    Or { addr: MirBitAddress },
    /// `X op`
    Xor { addr: MirBitAddress },
    /// `= dst`
    WriteBit { addr: MirBitAddress },
    /// `S dst`
    SetBit { addr: MirBitAddress },
    /// `R dst`
    ResetBit { addr: MirBitAddress },
    /// `U(`
    AndStart,
    /// `O(`
    OrStart,
    /// `X(`
    XorStart,
    /// `)`
    End,
}

impl Debug for MirInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Set => write!(f, "SET"),
            Self::Clear => write!(f, "CLR"),
            Self::Not => write!(f, "N"),
            Self::And { addr } => write!(f, "U {addr:?}"),
            Self::Or { addr } => write!(f, "O {addr:?}"),
            Self::Xor { addr } => write!(f, "X {addr:?}"),
            Self::WriteBit { addr } => write!(f, "= {addr:?}"),
            Self::SetBit { addr } => write!(f, "S {addr:?}"),
            Self::ResetBit { addr } => write!(f, "R {addr:?}"),
            Self::AndStart => write!(f, "U("),
            Self::OrStart => write!(f, "O("),
            Self::XorStart => write!(f, "X("),
            Self::End => write!(f, ")"),
        }
    }
}
