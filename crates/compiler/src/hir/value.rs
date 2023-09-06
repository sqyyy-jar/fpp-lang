use crate::util::Quote;

#[derive(Debug)]
pub struct HirValue {
    pub quote: Quote,
    pub r#type: HirValueType,
}

impl HirValue {
    pub fn new(quote: Quote, r#type: HirValueType) -> Self {
        Self { quote, r#type }
    }
}

#[derive(Debug)]
pub enum HirValueType {
    Number(HirNumber),
    Bool(HirBool),
    BitAddress(HirBitAddress),
    Not(Box<HirNot>),
    And(Box<HirAnd>),
    Or(Box<HirOr>),
    Xor(Box<HirXor>),
    VarRef(HirVarRef),
    Call(HirCall),
}

/// `true`, `false`
#[derive(Debug)]
pub struct HirBool {
    pub value: bool,
}

#[derive(Debug)]
pub struct HirNumber {
    pub value: usize,
}

/// `charptr.bit`
#[derive(Debug)]
pub struct HirBitAddress {
    pub char: u8,
    pub ptr: u16,
    pub bit: u8,
}

/// `not value`, `!value`
#[derive(Debug)]
pub struct HirNot {
    pub value: HirValue,
}

/// `left and right`, `left & right`
#[derive(Debug)]
pub struct HirAnd {
    pub left: HirValue,
    pub right: HirValue,
}

/// `left or right`, `left | right`
#[derive(Debug)]
pub struct HirOr {
    pub left: HirValue,
    pub right: HirValue,
}

/// `left xor right`, `left ^ right`
#[derive(Debug)]
pub struct HirXor {
    pub left: HirValue,
    pub right: HirValue,
}

/// `quote`
#[derive(Debug)]
pub struct HirVarRef;

/// `name(args, ...)`
#[derive(Debug)]
pub struct HirCall {
    pub name: Quote,
    pub args: Vec<HirValue>,
}
