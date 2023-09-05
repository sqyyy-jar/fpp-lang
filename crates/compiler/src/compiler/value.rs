use std::rc::Rc;

use crate::{
    error::Result,
    hir::value::{
        HirAddress, HirAnd, HirBool, HirCall, HirInput, HirNot, HirNumber, HirOr, HirOutput,
        HirValue, HirValueType, HirVarRef, HirXor,
    },
    mir::{
        value::{MirAnd, MirBool, MirNot, MirNumber, MirOr, MirValue, MirXor},
        Mir,
    },
};

fn compile_number(number: HirNumber) -> Result<MirValue> {
    Ok(MirValue::Number(MirNumber {
        value: number.value,
    }))
}

fn compile_bool(bool: HirBool) -> Result<MirValue> {
    Ok(MirValue::Bool(MirBool { value: bool.value }))
}

fn compile_address(_mir: &mut Mir, _address: HirAddress) -> Result<MirValue> {
    todo!()
}

fn compile_not(mir: &mut Mir, not: HirNot) -> Result<MirValue> {
    Ok(MirValue::Not(Rc::new(MirNot {
        value: compile_value(mir, not.value)?,
    })))
}

fn compile_and(mir: &mut Mir, and: HirAnd) -> Result<MirValue> {
    Ok(MirValue::And(Rc::new(MirAnd {
        left: compile_value(mir, and.left)?,
        right: compile_value(mir, and.right)?,
    })))
}

fn compile_or(mir: &mut Mir, or: HirOr) -> Result<MirValue> {
    Ok(MirValue::Or(Rc::new(MirOr {
        left: compile_value(mir, or.left)?,
        right: compile_value(mir, or.right)?,
    })))
}

fn compile_xor(mir: &mut Mir, xor: HirXor) -> Result<MirValue> {
    Ok(MirValue::Xor(Rc::new(MirXor {
        left: compile_value(mir, xor.left)?,
        right: compile_value(mir, xor.right)?,
    })))
}

fn compile_input(_mir: &mut Mir, _input: HirInput) -> Result<MirValue> {
    todo!()
}

fn compile_output(_mir: &mut Mir, _output: HirOutput) -> Result<MirValue> {
    todo!()
}

fn compile_var_ref(_mir: &mut Mir, _var: HirVarRef) -> Result<MirValue> {
    todo!()
}

fn compile_call(_mir: &mut Mir, _call: HirCall) -> Result<MirValue> {
    todo!()
}

pub(super) fn compile_value(mir: &mut Mir, value: HirValue) -> Result<MirValue> {
    match value.r#type {
        HirValueType::Number(number) => compile_number(number),
        HirValueType::Bool(bool) => compile_bool(bool),
        HirValueType::Address(address) => compile_address(mir, address),
        HirValueType::Not(not) => compile_not(mir, *not),
        HirValueType::And(and) => compile_and(mir, *and),
        HirValueType::Or(or) => compile_or(mir, *or),
        HirValueType::Xor(xor) => compile_xor(mir, *xor),
        HirValueType::Input(input) => compile_input(mir, input),
        HirValueType::Output(output) => compile_output(mir, output),
        HirValueType::VarRef(var) => compile_var_ref(mir, var),
        HirValueType::Call(call) => compile_call(mir, call),
    }
}
