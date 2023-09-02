pub mod value;

use crate::util::Quote;

use self::value::HirValue;

#[derive(Debug, Default)]
pub struct Hir {
    pub statements: Vec<HirStatement>,
}

#[derive(Debug)]
pub enum HirStatement {
    Let(HirLet),
    Write(HirWrite),
}

/// `let name = value;`
#[derive(Debug)]
pub struct HirLet {
    pub quote: Quote,
    pub name: Quote,
    pub value: HirValue,
}

/// `name = value;`
#[derive(Debug)]
pub struct HirWrite {
    pub quote: Quote,
    pub name: Quote,
    pub value: HirValue,
}
