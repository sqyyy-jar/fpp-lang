use crate::util::Quote;

#[derive(Debug)]
pub enum HirValue {
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

impl HirValue {
    pub fn quote(&self) -> Quote {
        match self {
            HirValue::Number(number) => number.quote.clone(),
            HirValue::Bool(bool) => bool.quote.clone(),
            HirValue::Address(address) => address.quote.clone(),
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
#[derive(Debug)]
pub struct HirBool {
    pub quote: Quote,
    pub value: bool,
}

#[derive(Debug)]
pub struct HirNumber {
    pub quote: Quote,
}

#[derive(Debug)]
pub struct HirAddress {
    pub quote: Quote,
    pub char: u8,
    pub x: usize,
    pub y: usize,
}

/// `not value`, `!value`
#[derive(Debug)]
pub struct HirNot {
    pub quote: Quote,
    pub value: HirValue,
}

/// `left and right`, `left & right`
#[derive(Debug)]
pub struct HirAnd {
    pub quote: Quote,
    pub left: HirValue,
    pub right: HirValue,
}

/// `left or right`, `left | right`
#[derive(Debug)]
pub struct HirOr {
    pub quote: Quote,
    pub left: HirValue,
    pub right: HirValue,
}

/// `left xor right`, `left ^ right`
#[derive(Debug)]
pub struct HirXor {
    pub quote: Quote,
    pub left: HirValue,
    pub right: HirValue,
}

/// `Ex.y`
#[derive(Debug)]
pub struct HirInput {
    pub quote: Quote,
    pub x: usize,
    pub y: usize,
}

/// `Ax.y`
#[derive(Debug)]
pub struct HirOutput {
    pub quote: Quote,
    pub x: usize,
    pub y: usize,
}

/// `quote`
#[derive(Debug)]
pub struct HirVariable {
    pub quote: Quote,
}

/// `name(args, ...)`
#[derive(Debug)]
pub struct HirCall {
    pub quote: Quote,
    pub name: Quote,
    pub args: Vec<HirValue>,
}
