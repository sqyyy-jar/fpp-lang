//! This module is responsible for compiling a [Hir] into an [Mir].

pub mod value;

use crate::{
    error::{Error, Reason, Result},
    hir::{Hir, HirCallStatement, HirLetStatement, HirStatement, HirWriteStatement},
    mir::{transformer::value::transform_values, Mir, MirVariable, BUILTIN_FUNCTIONS},
};

use self::value::transform_value;

use super::{
    writer::{optimizer::optimize, MirInstructionWriter},
    MirAction, MirRawAction,
};

fn transform_let(
    mir: &mut Mir,
    HirLetStatement { name, value, .. }: HirLetStatement,
) -> Result<()> {
    let mir_value = transform_value(mir, value)?;
    mir.variables.push(MirVariable {
        name,
        value: mir_value,
    });
    Ok(())
}

fn transform_write(
    mir: &mut Mir,
    HirWriteStatement { quote, name, value }: HirWriteStatement,
) -> Result<()> {
    let write_name = &mir.source.code[&name];
    let Some(index) = mir.find_var(write_name) else {
        return Err(Error::new(
            mir.source.clone(),
            quote,
            Reason::UnknownVariable,
        ));
    };
    let var_value = mir.variables[index].value.clone();
    let value_quote = value.quote.clone();
    let mir_value = transform_value(mir, value)?;
    var_value.write(mir, name, value_quote, mir_value)?;
    mir.variables[index].value = var_value;
    Ok(())
}

fn transform_call(mir: &mut Mir, call: HirCallStatement) -> Result<()> {
    let function_name = &mir.source.code[&call.name];
    let mut value = None;
    if let Some(func) = BUILTIN_FUNCTIONS.get(function_name) {
        let args = transform_values(mir, call.args)?;
        value = Some(func(mir, call.quote.clone(), &args)?);
    };
    let Some(value) = value else {
        return Err(Error::new(
            mir.source.clone(),
            call.name,
            Reason::UnknownFunction,
        ));
    };
    if !value.is_bit_readable(mir) {
        return Err(Error::new(
            mir.source.clone(),
            call.quote,
            Reason::ValueNotBitReadable,
        ));
    }
    let mut writer = MirInstructionWriter::default();
    writer.write_value(mir, &value)?;
    optimize(&mut writer);
    mir.actions.push(MirAction::Raw(MirRawAction {
        instructions: writer.instructions,
    }));
    Ok(())
}

pub fn transform(hir: Hir) -> Result<Mir> {
    let mut mir = Mir::new(hir.source.clone());
    for statement in hir.statements {
        match statement {
            HirStatement::Let(stmt) => transform_let(&mut mir, stmt)?,
            HirStatement::Write(write) => transform_write(&mut mir, write)?,
            HirStatement::Call(call) => transform_call(&mut mir, call)?,
        }
    }
    Ok(mir)
}
