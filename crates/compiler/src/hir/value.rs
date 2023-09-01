use crate::util::Quote;

pub enum HirValue {
    Number(HirNumber),
    Bool(HirBool),
    Not(Box<HirNot>),
    And(Box<HirAnd>),
    Or(Box<HirOr>),
    Xor(Box<HirXor>),
    Input(HirInput),
    Output(HirOutput),
    Variable(HirVariable),
    Call(HirCall),
}

impl HirValue {
    pub fn quote(&self) -> Quote {
        match self {
            HirValue::Number(number) => number.quote.clone(),
            HirValue::Bool(bool) => bool.quote.clone(),
            HirValue::Not(not) => not.quote.clone(),
            HirValue::And(and) => and.quote.clone(),
            HirValue::Or(or) => or.quote.clone(),
            HirValue::Xor(xor) => xor.quote.clone(),
            HirValue::Input(input) => input.quote.clone(),
            HirValue::Output(output) => output.quote.clone(),
            HirValue::Variable(variable) => variable.quote.clone(),
            HirValue::Call(call) => call.quote.clone(),
        }
    }
}

/// `true`, `false`
pub struct HirBool {
    pub quote: Quote,
    pub value: bool,
}

pub struct HirNumber {
    pub quote: Quote,
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
