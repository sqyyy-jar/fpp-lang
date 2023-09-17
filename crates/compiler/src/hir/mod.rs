pub mod value;

use std::rc::Rc;

use crate::util::{Quote, Source};

use self::value::HirValue;

#[derive(Debug)]
pub struct Hir {
    pub source: Rc<Source>,
    pub statements: Vec<HirStatement>,
}

impl Hir {
    pub fn new(source: Rc<Source>) -> Self {
        Self {
            source,
            statements: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum HirStatement {
    Let(HirLetStatement),
    Write(HirWriteStatement),
    Call(HirCallStatement),
}

/// `let name = value;`
#[derive(Debug)]
pub struct HirLetStatement {
    pub quote: Quote,
    pub name: Quote,
    pub value: HirValue,
}

/// `name = value;`
#[derive(Debug)]
pub struct HirWriteStatement {
    pub quote: Quote,
    pub name: Quote,
    pub value: HirValue,
}

/// `abc();`
#[derive(Debug)]
pub struct HirCallStatement {
    pub quote: Quote,
    pub name: Quote,
    pub args: Vec<HirValue>,
}
