pub mod value;

use crate::util::Quote;

use self::value::HirValue;

#[derive(Default)]
pub struct Hir {
    pub statements: Vec<HirStatement>,
}

pub enum HirStatement {
    Let(HirLet),
    Write(HirWrite),
}

/// `let name = value;`
pub struct HirLet {
    pub quote: Quote,
    pub name: Quote,
    pub value: HirValue,
}

/// `name = value;`
pub struct HirWrite {
    pub quote: Quote,
    pub name: Quote,
    pub value: HirValue,
}
