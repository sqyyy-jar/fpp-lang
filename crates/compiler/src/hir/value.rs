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
    Address(HirAddress),
    Not(Box<HirNot>),
    And(Box<HirAnd>),
    Or(Box<HirOr>),
    Xor(Box<HirXor>),
    Input(HirInput),
    Output(HirOutput),
    Variable(HirVariable),
    Call(HirCall),
}

/// `true`, `false`
#[derive(Debug)]
pub struct HirBool {
    pub value: bool,
}

#[derive(Debug)]
pub struct HirNumber;

/// `charx.y`
#[derive(Debug)]
pub struct HirAddress {
    pub char: u8,
    pub x: usize,
    pub y: usize,
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

/// `Ex.y`
#[derive(Debug)]
pub struct HirInput {
    pub x: usize,
    pub y: usize,
}

/// `Ax.y`
#[derive(Debug)]
pub struct HirOutput {
    pub x: usize,
    pub y: usize,
}

/// `quote`
#[derive(Debug)]
pub struct HirVariable;

/// `name(args, ...)`
#[derive(Debug)]
pub struct HirCall {
    pub name: Quote,
    pub args: Vec<HirValue>,
}
