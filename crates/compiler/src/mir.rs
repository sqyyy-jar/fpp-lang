pub struct Mir {
    pub networks: Vec<MirNetwork>,
}

pub struct MirNetwork {
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

#[repr(u8)]
pub enum MirBitAddressType {
    Input,
    Output,
    Memory,
}

pub struct MirBitAddress {
    pub r#type: MirBitAddressType,
    pub x: u16,
    pub y: u8,
}
