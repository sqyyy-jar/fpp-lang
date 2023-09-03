pub struct S7Lir {
    pub networks: Vec<S7Network>,
}

pub struct S7Network {
    pub instructions: Vec<S7Instruction>,
}

pub enum S7Instruction {
    /// `SET`
    Set,
    /// `CLR`
    Clear,
    /// `N`
    Not,
    /// `U op`
    And { op: S7BitAddress },
    /// `O op`
    Or { op: S7BitAddress },
    /// `X op`
    Xor { op: S7BitAddress },
    /// `= dst`
    WriteBit { dst: S7BitAddress },
    /// `S dst`
    SetBit { dst: S7BitAddress },
    /// `R dst`
    ResetBit { dst: S7BitAddress },
}

#[repr(u8)]
pub enum S7BitAddressType {
    Input,
    Output,
    Memory,
}

pub struct S7BitAddress {
    pub r#type: S7BitAddressType,
    pub x: u16,
    pub y: u8,
}
