//! This module is responsible for compiling a [Hir] into an [Mir].

use crate::{
    error::{Error, Reason, Result},
    hir::{
        value::{HirValue, HirValueType},
        Hir, HirStatement,
    },
    mir::{value::MirValue, Mir, MirVariable},
};

fn compile_value(_mir: &mut Mir, value: HirValue) -> Result<MirValue> {
    match value.r#type {
        HirValueType::Number(_) => todo!(),
        HirValueType::Bool(_) => todo!(),
        HirValueType::Address(_) => todo!(),
        HirValueType::Not(_) => todo!(),
        HirValueType::And(_) => todo!(),
        HirValueType::Or(_) => todo!(),
        HirValueType::Xor(_) => todo!(),
        HirValueType::Input(_) => todo!(),
        HirValueType::Output(_) => todo!(),
        HirValueType::Variable(_) => todo!(),
        HirValueType::Call(_) => todo!(),
    }
}

pub fn compile(hir: Hir) -> Result<Mir> {
    let mut mir = Mir::new(hir.source.clone());
    for statement in hir.statements {
        match statement {
            HirStatement::Let(stmt) => {
                let mir_value = compile_value(&mut mir, stmt.value)?;
                mir.variables.push(MirVariable {
                    name: stmt.name,
                    value: mir_value,
                })
            }
            HirStatement::Write(write) => {
                let write_name = &hir.source[&write.name];
                let mut found = false;
                let mut index = 0;
                for (i, var) in mir.variables.iter().enumerate().rev() {
                    let var_name = &hir.source[&var.name];
                    if write_name != var_name {
                        continue;
                    }
                    found = true;
                    index = i;
                    break;
                }
                if !found {
                    return Err(Error::new(
                        hir.source.clone(),
                        write.quote,
                        Reason::UnknownVariable,
                    ));
                }
                let mut value = mir.variables[index].value.clone();
                let mir_value = compile_value(&mut mir, write.value)?;
                value.write(&mut mir, write.quote, index, mir_value)?;
                mir.variables[index].value = value;
            }
        }
    }
    Ok(mir)
}
