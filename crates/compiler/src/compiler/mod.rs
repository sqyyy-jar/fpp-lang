//! This module is responsible for compiling a [Hir] into an [Mir].

pub mod value;

use crate::{
    error::{Error, Reason, Result},
    hir::{Hir, HirLet, HirStatement, HirWrite},
    mir::{Mir, MirVariable},
};

use self::value::compile_value;

fn compile_let(mir: &mut Mir, HirLet { name, value, .. }: HirLet) -> Result<()> {
    let mir_value = compile_value(mir, value)?;
    mir.variables.push(MirVariable {
        name,
        value: mir_value,
    });
    Ok(())
}

fn compile_write(mir: &mut Mir, HirWrite { quote, name, value }: HirWrite) -> Result<()> {
    let write_name = &mir.source[&name];
    let mut index = None;
    for (i, var) in mir.variables.iter().enumerate().rev() {
        let var_name = &mir.source[&var.name];
        if write_name != var_name {
            continue;
        }
        index = Some(i);
        break;
    }
    let Some(index) = index else {
        return Err(Error::new(
            mir.source.clone(),
            quote,
            Reason::UnknownVariable,
        ));
    };
    let var_value = mir.variables[index].value.clone();
    let mir_value = compile_value(mir, value)?;
    var_value.write(mir, quote, index, mir_value)?;
    mir.variables[index].value = var_value;
    Ok(())
}

pub fn compile(hir: Hir) -> Result<Mir> {
    let mut mir = Mir::new(hir.source.clone());
    for statement in hir.statements {
        match statement {
            HirStatement::Let(stmt) => compile_let(&mut mir, stmt)?,
            HirStatement::Write(write) => compile_write(&mut mir, write)?,
        }
    }
    Ok(mir)
}
