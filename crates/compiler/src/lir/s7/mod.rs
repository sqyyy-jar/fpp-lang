use std::io::Write;

pub mod transformer;

/// Thing that can be written to a [Write] in AWL-form
pub trait WriteAwl {
    fn write_awl(&self, out: &mut impl Write) -> std::io::Result<()>;
}

#[derive(Debug, Default)]
pub struct S7Lir {
    pub networks: Vec<S7Network>,
}

#[derive(Debug, Default)]
pub struct S7Network {
    pub instructions: Vec<S7Instruction>,
}

impl WriteAwl for S7Network {
    fn write_awl(&self, out: &mut impl Write) -> std::io::Result<()> {
        for instruction in &self.instructions {
            instruction.write_awl(out)?;
            write!(out, "\n")?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum S7Instruction {
    /// `U op`
    And { addr: S7Address },
    /// `UN op`
    AndNot { addr: S7Address },
    /// `O op`
    Or { addr: S7Address },
    /// `ON op`
    OrNot { addr: S7Address },
    /// `X op`
    Xor { addr: S7Address },
    /// `XN op`
    XorNot { addr: S7Address },
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
    AssignBit { addr: S7Address },
    /// `R dst`
    ResetBit { addr: S7Address },
    /// `S dst`
    SetBit { addr: S7Address },
    /// `N`
    Not,
    /// `SET`
    Set,
    /// `CLR`
    Clear,
    /// `SAVE`
    Save,
    /// `L addr`
    CounterLoadInt { addr: S7Address },
    /// `LC addr`
    CounterLoadBcd { addr: S7Address },
    /// `R addr`
    CounterReset { addr: S7Address },
    /// `S addr`
    CounterSet { addr: S7Address },
    /// `ZV addr`
    CounterForward { addr: S7Address },
    /// `ZR addr`
    CounterBackward { addr: S7Address },
    /// `T addr`
    Transfer { addr: S7Address },
}

impl S7Instruction {
    pub fn awl_mnemonic(self) -> &'static str {
        match self {
            Self::And { .. } => "U",
            Self::AndNot { .. } => "UN",
            Self::Or { .. } => "O",
            Self::OrNot { .. } => "ON",
            Self::Xor { .. } => "X",
            Self::XorNot { .. } => "XN",
            Self::AndStart => "U(",
            Self::AndNotStart => "UN(",
            Self::OrStart => "O(",
            Self::OrNotStart => "ON(",
            Self::XorStart => "X(",
            Self::XorNotStart => "XN(",
            Self::End => ")",
            Self::AssignBit { .. } => "=",
            Self::ResetBit { .. } => "R",
            Self::SetBit { .. } => "S",
            Self::Not => "N",
            Self::Set => "SET",
            Self::Clear => "CLR",
            Self::Save => "SAVE",
            Self::CounterLoadInt { .. } => "L",
            Self::CounterLoadBcd { .. } => "LC",
            Self::CounterReset { .. } => "R",
            Self::CounterSet { .. } => "S",
            Self::CounterForward { .. } => "ZV",
            Self::CounterBackward { .. } => "ZR",
            Self::Transfer { .. } => "T",
        }
    }

    pub fn addr(self) -> Option<S7Address> {
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
            | Self::Transfer { addr } => Some(addr),
            _ => None,
        }
    }
}

impl WriteAwl for S7Instruction {
    fn write_awl(&self, out: &mut impl Write) -> std::io::Result<()> {
        let mnemonic = self.awl_mnemonic();
        write!(out, "{mnemonic}")?;
        if let Some(addr) = self.addr() {
            write!(out, " ")?;
            addr.write_awl(out)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum S7AddressType {
    Input1,
    Output1,
    Memory1,
    Memory8,
    Memory16,
    Memory32,
    Counter,
}

impl WriteAwl for S7AddressType {
    fn write_awl(&self, out: &mut impl Write) -> std::io::Result<()> {
        let prefix = match self {
            Self::Input1 => "E",
            Self::Output1 => "A",
            Self::Memory1 => "M",
            Self::Memory8 => "MB",
            Self::Memory16 => "MW",
            Self::Memory32 => "MD",
            Self::Counter => "C",
        };
        write!(out, "{prefix}")
    }
}

#[derive(Clone, Copy, Debug)]
pub struct S7Address {
    pub r#type: S7AddressType,
    pub ptr: u16,
    pub bit: u8,
}

impl S7Address {
    /// Checks if the address points to a `BIT`
    pub fn is_bit_address(self) -> bool {
        matches!(
            self.r#type,
            S7AddressType::Input1 | S7AddressType::Output1 | S7AddressType::Memory1
        )
    }

    /// Checks if the address points to a `BYTE`, `WORD` or `DWORD`
    pub fn is_any_byte_address(self) -> bool {
        matches!(
            self.r#type,
            S7AddressType::Memory8 | S7AddressType::Memory16 | S7AddressType::Memory32
        )
    }
}

impl WriteAwl for S7Address {
    fn write_awl(&self, out: &mut impl Write) -> std::io::Result<()> {
        self.r#type.write_awl(out)?;
        if self.is_bit_address() {
            write!(out, "{}.{}", self.ptr, self.bit)
        } else {
            write!(out, "{}", self.ptr)
        }
    }
}
