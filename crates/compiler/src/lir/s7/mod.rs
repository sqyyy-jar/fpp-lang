pub mod transformer;

#[derive(Debug, Default)]
pub struct S7Lir {
    pub networks: Vec<S7Network>,
}

#[derive(Debug, Default)]
pub struct S7Network {
    pub instructions: Vec<S7Instruction>,
}

#[derive(Debug)]
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

#[derive(Clone, Copy, Debug)]
pub struct S7Address {
    pub r#type: S7AddressType,
    pub ptr: u16,
    pub bit: u8,
}
