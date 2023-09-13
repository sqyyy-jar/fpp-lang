use std::{fmt::Debug, rc::Rc};

use phf::{phf_map, Map};

use crate::{error::Result, util::Quote};

use self::{
    builtin::{
        counter::builtin_counter,
        flipflop::{builtin_rs, builtin_sr},
        memory::{builtin_alloc1, builtin_alloc16, builtin_alloc32, builtin_alloc8},
    },
    value::{MirAddress, MirAddressType, MirValue},
};

pub mod builtin;
pub mod ops;
pub mod transformer;
pub mod value;
pub mod writer;

pub const BUILTIN_FUNCTIONS: Map<&[u8], MirFunction> = phf_map! {
    b"M" => builtin_alloc1,
    b"alloc1" => builtin_alloc1,
    b"MB" => builtin_alloc8,
    b"alloc8" => builtin_alloc8,
    b"MW" => builtin_alloc16,
    b"alloc16" => builtin_alloc16,
    b"MD" => builtin_alloc32,
    b"alloc32" => builtin_alloc32,
    b"Z" => builtin_counter,
    b"counter" => builtin_counter,
    b"rs" => builtin_rs,
    b"sr" => builtin_sr,
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
    pub allocator: MirAllocator,
    pub variables: Vec<MirVariable>,
    pub actions: Vec<MirAction>,
}

impl Mir {
    pub fn new(source: Rc<[u8]>) -> Self {
        Self {
            source,
            allocator: MirAllocator::default(),
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
pub struct MirAllocator {
    pub allocated_bits: usize,
    pub allocated_bytes: usize,
    pub allocated_counters: u16,
}

impl MirAllocator {
    pub fn byte_offset(&self) -> usize {
        if self.allocated_bits % 8 != 0 {
            self.allocated_bits / 8 + 1
        } else {
            self.allocated_bits / 8
        }
    }

    pub fn usage(&self) -> usize {
        self.byte_offset() + self.allocated_bytes
    }

    pub fn can_alloc_bit(&self) -> bool {
        self.allocated_bits % 8 != 0 || self.usage() < 65535
    }

    pub fn alloc1(&mut self) -> Option<MirAddress> {
        if !self.can_alloc_bit() {
            return None;
        }
        let ptr = self.allocated_bits;
        self.allocated_bits += 1;
        Some(MirAddress {
            r#type: value::MirAddressType::Memory1,
            ptr: (ptr / 8) as u16,
            bit: (ptr % 8) as u8,
        })
    }

    pub fn alloc8(&mut self) -> Option<MirAddress> {
        if self.usage() >= 65535 {
            return None;
        }
        let ptr = self.allocated_bytes;
        self.allocated_bytes += 1;
        Some(MirAddress {
            r#type: value::MirAddressType::Memory8,
            ptr: ptr as u16,
            bit: 0,
        })
    }

    pub fn alloc16(&mut self) -> Option<MirAddress> {
        if self.usage() >= 65534 {
            return None;
        }
        let ptr = self.allocated_bytes;
        self.allocated_bytes += 2;
        Some(MirAddress {
            r#type: value::MirAddressType::Memory16,
            ptr: ptr as u16,
            bit: 0,
        })
    }

    pub fn alloc32(&mut self) -> Option<MirAddress> {
        if self.usage() >= 65532 {
            return None;
        }
        let ptr = self.allocated_bytes;
        self.allocated_bytes += 4;
        Some(MirAddress {
            r#type: value::MirAddressType::Memory32,
            ptr: ptr as u16,
            bit: 0,
        })
    }

    pub fn alloc_counter(&mut self) -> Option<MirAddress> {
        let ptr = self.allocated_counters;
        if ptr >= 65535 {
            return None;
        }
        self.allocated_counters += 1;
        Some(MirAddress {
            r#type: MirAddressType::PhysicalCounter,
            ptr,
            bit: 0,
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
    Raw(MirRawAction),
    Output(MirOutputAction),
}

#[derive(Debug)]
pub struct MirRawAction {
    pub instructions: Vec<MirInstruction>,
}

#[derive(Debug)]
pub struct MirOutputAction {
    pub address: MirAddress,
    pub instructions: Vec<MirInstruction>,
}

#[derive(Clone, Copy)]
pub enum MirInstruction {
    /// Dummy instruction
    Dummy,
    /// `U op`
    And { addr: MirAddress },
    /// `UN op`
    AndNot { addr: MirAddress },
    /// `O op`
    Or { addr: MirAddress },
    /// `ON op`
    OrNot { addr: MirAddress },
    /// `X op`
    Xor { addr: MirAddress },
    /// `XN op`
    XorNot { addr: MirAddress },
    /// `U(`
    AndStart,
    /// `UN(`
    AndNotStart,
    /// `O(`
    OrStart,
    /// `ON(`
    OrNotStart,
    /// `X(`
    XorStart,
    /// `XN(`
    XorNotStart,
    /// `)`
    End,
    /// `= dst`
    AssignBit { addr: MirAddress },
    /// `R dst`
    ResetBit { addr: MirAddress },
    /// `S dst`
    SetBit { addr: MirAddress },
    /// `N`
    Not,
    /// `SET`
    Set,
    /// `CLR`
    Clear,
    /// `SAVE`
    Save,
    /// `L addr`
    CounterLoadInt { addr: MirAddress },
    /// `LC addr`
    CounterLoadBcd { addr: MirAddress },
    /// `R addr`
    CounterReset { addr: MirAddress },
    /// `S addr`
    CounterSet { addr: MirAddress },
    /// `ZV addr`
    CounterForward { addr: MirAddress },
    /// `ZR addr`
    CounterBackward { addr: MirAddress },
    /// `T addr`
    Transfer { addr: MirAddress },
}

impl MirInstruction {
    pub fn is_dummy(&self) -> bool {
        matches!(self, Self::Dummy)
    }

    pub fn is_end(&self) -> bool {
        matches!(self, Self::End)
    }

    pub fn has_bracket(&self) -> bool {
        matches!(
            self,
            Self::AndStart
                | Self::AndNotStart
                | Self::OrStart
                | Self::OrNotStart
                | Self::XorStart
                | Self::XorNotStart
        )
    }

    pub fn unbracket(&self, addr: MirAddress) -> MirInstruction {
        match self {
            Self::AndStart => Self::And { addr },
            Self::AndNotStart => Self::AndNot { addr },
            Self::OrStart => Self::Or { addr },
            Self::OrNotStart => Self::OrNot { addr },
            Self::XorStart => Self::Xor { addr },
            Self::XorNotStart => Self::XorNot { addr },
            _ => panic!("Invalid instruction"),
        }
    }

    pub fn addr(&self) -> MirAddress {
        match self {
            Self::And { addr }
            | Self::AndNot { addr }
            | Self::Or { addr }
            | Self::OrNot { addr }
            | Self::Xor { addr }
            | Self::XorNot { addr }
            | Self::AssignBit { addr }
            | Self::ResetBit { addr }
            | Self::SetBit { addr }
            | Self::CounterLoadInt { addr }
            | Self::CounterLoadBcd { addr }
            | Self::CounterReset { addr }
            | Self::CounterSet { addr }
            | Self::CounterForward { addr }
            | Self::CounterBackward { addr }
            | Self::Transfer { addr } => *addr,
            _ => panic!("Invalid instruction"),
        }
    }
}

impl Debug for MirInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dummy => write!(f, "DUMMY"),
            Self::And { addr } => write!(f, "U {addr:?}"),
            Self::AndNot { addr } => write!(f, "UN {addr:?}"),
            Self::Or { addr } => write!(f, "O {addr:?}"),
            Self::OrNot { addr } => write!(f, "ON {addr:?}"),
            Self::Xor { addr } => write!(f, "X {addr:?}"),
            Self::XorNot { addr } => write!(f, "XN {addr:?}"),
            Self::AndStart => write!(f, "U("),
            Self::AndNotStart => write!(f, "UN("),
            Self::OrStart => write!(f, "O("),
            Self::OrNotStart => write!(f, "ON("),
            Self::XorStart => write!(f, "X("),
            Self::XorNotStart => write!(f, "XN("),
            Self::End => write!(f, ")"),
            Self::AssignBit { addr } => write!(f, "= {addr:?}"),
            Self::ResetBit { addr } => write!(f, "R {addr:?}"),
            Self::SetBit { addr } => write!(f, "S {addr:?}"),
            Self::Not => write!(f, "N"),
            Self::Set => write!(f, "SET"),
            Self::Clear => write!(f, "CLR"),
            Self::Save => write!(f, "SAVE"),
            Self::CounterLoadInt { addr } => write!(f, "L {addr:?}"),
            Self::CounterLoadBcd { addr } => write!(f, "LC {addr:?}"),
            Self::CounterReset { addr } => write!(f, "R {addr:?}"),
            Self::CounterSet { addr } => write!(f, "S {addr:?}"),
            Self::CounterForward { addr } => write!(f, "ZV {addr:?}"),
            Self::CounterBackward { addr } => write!(f, "ZR {addr:?}"),
            Self::Transfer { addr } => write!(f, "T {addr:?}"),
        }
    }
}
