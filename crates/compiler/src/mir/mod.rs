use std::{fmt::Debug, rc::Rc};

use phf::{phf_map, Map};

use crate::{error::Result, util::Quote};

use self::value::{MirAddress, MirValue};

pub mod builtin;
pub mod ops;
pub mod transformer;
pub mod value;
pub mod writer;

pub const BUILTIN_FUNCTIONS: Map<&[u8], MirFunction> = phf_map! {
    b"M" => builtin::memory::builtin_m,
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
    pub allocated_bits: usize,
    pub allocated_bytes: usize,
}

impl MirMemory {
    pub fn alloc1(&mut self) -> Option<MirAddress> {
        let ptr = self.allocated_bits;
        if ptr == 0xffff_ffff {
            return None;
        }
        self.allocated_bits += 1;
        Some(MirAddress {
            r#type: value::MirAddressType::Memory1,
            ptr: ptr as u32,
        })
    }

    pub fn alloc8(&mut self) -> Option<MirAddress> {
        let ptr = self.allocated_bytes;
        if ptr == 0xffff_ffff {
            return None;
        }
        self.allocated_bytes += 1;
        Some(MirAddress {
            r#type: value::MirAddressType::Memory8,
            ptr: ptr as u32,
        })
    }

    pub fn alloc16(&mut self) -> Option<MirAddress> {
        let ptr = self.allocated_bytes;
        if ptr == 0xffff_ffff {
            return None;
        }
        self.allocated_bytes += 2;
        Some(MirAddress {
            r#type: value::MirAddressType::Memory16,
            ptr: ptr as u32,
        })
    }

    pub fn alloc32(&mut self) -> Option<MirAddress> {
        let ptr = self.allocated_bytes;
        if ptr == 0xffff_ffff {
            return None;
        }
        self.allocated_bytes += 4;
        Some(MirAddress {
            r#type: value::MirAddressType::Memory32,
            ptr: ptr as u32,
        })
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
    pub address: MirAddress,
    pub instructions: Vec<MirInstruction>,
}

#[derive(Clone)]
pub enum MirInstruction {
    /// Dummy instruction
    Dummy,
    /// `SET`
    Set,
    /// `CLR`
    Clear,
    /// `N`
    Not,
    /// `U op`
    And { addr: MirAddress },
    /// `O op`
    Or { addr: MirAddress },
    /// `X op`
    Xor { addr: MirAddress },
    /// `= dst`
    WriteBit { addr: MirAddress },
    /// `S dst`
    SetBit { addr: MirAddress },
    /// `R dst`
    ResetBit { addr: MirAddress },
    /// `U(`
    AndStart,
    /// `O(`
    OrStart,
    /// `X(`
    XorStart,
    /// `)`
    End,
}

impl MirInstruction {
    pub fn is_dummy(&self) -> bool {
        matches!(self, Self::Dummy)
    }

    pub fn is_end(&self) -> bool {
        matches!(self, Self::End)
    }

    pub fn has_bracket(&self) -> bool {
        matches!(self, Self::AndStart | Self::OrStart | Self::XorStart)
    }

    pub fn unbracket(&self, addr: MirAddress) -> MirInstruction {
        match self {
            MirInstruction::AndStart => MirInstruction::And { addr },
            MirInstruction::OrStart => MirInstruction::Or { addr },
            MirInstruction::XorStart => MirInstruction::Xor { addr },
            _ => panic!("Invalid instruction"),
        }
    }

    pub fn addr(&self) -> MirAddress {
        match self {
            MirInstruction::And { addr }
            | MirInstruction::Or { addr }
            | MirInstruction::Xor { addr }
            | MirInstruction::WriteBit { addr }
            | MirInstruction::SetBit { addr }
            | MirInstruction::ResetBit { addr } => *addr,
            _ => panic!("Invalid instruction"),
        }
    }
}

impl Debug for MirInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dummy => write!(f, "DUMMY"),
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
