use crate::util::Quote;

pub enum HirValue {
    Not(Box<HirNot>),
    And(Box<HirAnd>),
    Or(Box<HirOr>),
    Xor(Box<HirXor>),
    Input(HirInput),
    Output(HirOutput),
    Variable(HirVariable),
    Call(HirCall),
}

/// `not value`, `!value`
pub struct HirNot {
    pub quote: Quote,
    pub value: HirValue,
}

/// `left and right`, `left & right`
pub struct HirAnd {
    pub quote: Quote,
    pub left: HirValue,
    pub right: HirValue,
}

/// `left or right`, `left | right`
pub struct HirOr {
    pub quote: Quote,
    pub left: HirValue,
    pub right: HirValue,
}

/// `left xor right`, `left ^ right`
pub struct HirXor {
    pub quote: Quote,
    pub left: HirValue,
    pub right: HirValue,
}

/// `Ex.y`
pub struct HirInput {
    pub quote: Quote,
    pub x: Quote,
    pub y: Quote,
}

/// `Ax.y`
pub struct HirOutput {
    pub quote: Quote,
    pub x: Quote,
    pub y: Quote,
}

/// `quote`
pub struct HirVariable {
    pub quote: Quote,
}

/// `name(args, ...)`
pub struct HirCall {
    pub quote: Quote,
    pub name: Quote,
    pub args: Vec<HirValue>,
}
