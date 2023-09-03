pub mod value;

use std::rc::Rc;

use crate::util::Quote;

use self::value::HirValue;

#[derive(Debug)]
pub struct Hir {
    pub source: Rc<[u8]>,
    pub statements: Vec<HirStatement>,
}

impl Hir {
    pub fn new(source: Rc<[u8]>) -> Self {
        Self {
            source,
            statements: Vec::new(),
        }
    }
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
