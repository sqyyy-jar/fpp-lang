use std::rc::Rc;

use crate::{
    error::{Error, Reason, Result},
    hir::value::{
        HirAnd, HirBitAddress, HirBool, HirCall, HirNot, HirNumber, HirOr, HirValue, HirValueType,
        HirXor,
    },
    mir::{
        value::{
            MirAnd, MirBitAddress, MirBitAddressType, MirBool, MirNot, MirNumber, MirOr, MirValue,
            MirVarRef, MirXor,
        },
        Mir, BUILTIN_FUNCTIONS,
    },
    util::Quote,
};

fn compile_values(mir: &mut Mir, hir_values: Vec<HirValue>) -> Result<Vec<MirValue>> {
    let mut mir_values = Vec::with_capacity(hir_values.len());
    for value in hir_values {
        mir_values.push(compile_value(mir, value)?);
    }
    Ok(mir_values)
}

fn compile_number(number: HirNumber) -> Result<MirValue> {
    Ok(MirValue::Number(MirNumber {
        value: number.value,
    }))
}

fn compile_bool(bool: HirBool) -> Result<MirValue> {
    Ok(MirValue::Bool(MirBool { value: bool.value }))
}

fn compile_address(
    mir: &mut Mir,
    quote: Quote,
    HirBitAddress { char, x, y }: HirBitAddress,
) -> Result<MirValue> {
    match char {
        b'E' => Ok(MirValue::BitAddress(MirBitAddress {
            r#type: MirBitAddressType::Input,
            x,
            y,
        })),
        b'A' => Ok(MirValue::BitAddress(MirBitAddress {
            r#type: MirBitAddressType::Output,
            x,
            y,
        })),
        b'M' => Ok(MirValue::BitAddress(MirBitAddress {
            r#type: MirBitAddressType::Memory,
            x,
            y,
        })),
        _ => Err(Error::new(
            mir.source.clone(),
            quote,
            Reason::UnknownBitAddressType,
        )),
    }
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

fn compile_var_ref(mir: &mut Mir, quote: Quote) -> Result<MirValue> {
    let var_name = &mir.source[&quote];
    let Some(index) = mir.find_var(var_name) else {
        return Err(Error::new(
            mir.source.clone(),
            quote,
            Reason::UnknownVariable,
        ));
    };
    Ok(MirValue::VarRef(MirVarRef { index }))
}

fn compile_call(mir: &mut Mir, quote: Quote, call: HirCall) -> Result<MirValue> {
    let function_name = &mir.source[&call.name];
    if let Some(func) = BUILTIN_FUNCTIONS.get(function_name) {
        let args = compile_values(mir, call.args)?;
        return func(mir, quote, &args);
    };
    Err(Error::new(
        mir.source.clone(),
        call.name,
        Reason::UnknownFunction,
    ))
}

pub(super) fn compile_value(mir: &mut Mir, value: HirValue) -> Result<MirValue> {
    match value.r#type {
        HirValueType::Number(number) => compile_number(number),
        HirValueType::Bool(bool) => compile_bool(bool),
        HirValueType::BitAddress(address) => compile_address(mir, value.quote, address),
        HirValueType::Not(not) => compile_not(mir, *not),
        HirValueType::And(and) => compile_and(mir, *and),
        HirValueType::Or(or) => compile_or(mir, *or),
        HirValueType::Xor(xor) => compile_xor(mir, *xor),
        HirValueType::VarRef(_) => compile_var_ref(mir, value.quote),
        HirValueType::Call(call) => compile_call(mir, value.quote, call),
    }
}
