use super::value::{MirBitAddress, MirValue};

#[derive(Debug)]
pub enum MirOp {
    SetBit { cond: MirValue, addr: MirBitAddress },
    ResetBit { cond: MirValue, addr: MirBitAddress },
    And { addr: MirBitAddress },
}
