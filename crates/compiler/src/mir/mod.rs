use std::rc::Rc;

use self::value::{MirBitAddress, MirValueBox};

pub mod value;

pub struct Mir {
    pub source: Rc<[u8]>,
    pub variables: Vec<MirVariable>,
    pub actions: Vec<MirAction>,
}

pub struct MirVariable {
    pub value: MirValueBox,
}

pub enum MirAction {
    Output(MirOutputAction),
}

pub struct MirOutputAction {
    pub address: MirBitAddress,
    pub instructions: Vec<()>,
}
