use super::value::{MirAddress, MirValue};

#[derive(Debug)]
pub enum MirOp {
    SetBit { cond: MirValue, addr: MirAddress },
    ResetBit { cond: MirValue, addr: MirAddress },
    And { addr: MirAddress },
}
