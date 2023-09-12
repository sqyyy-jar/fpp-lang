use std::rc::Rc;

use crate::{
    error::{Error, Reason, Result},
    hir::value::{
        HirAnd, HirBitAddress, HirBool, HirCall, HirNot, HirNumber, HirOr, HirValue, HirValueType,
        HirXor,
    },
    mir::{
        value::{
            MirAddress, MirAddressType, MirAnd, MirBool, MirNot, MirNumber, MirOr, MirValue,
            MirVarRef, MirXor,
        },
        Mir, BUILTIN_FUNCTIONS,
    },
    util::Quote,
};

pub fn transform_values(mir: &mut Mir, hir_values: Vec<HirValue>) -> Result<Vec<MirValue>> {
    let mut mir_values = Vec::with_capacity(hir_values.len());
    for value in hir_values {
        mir_values.push(transform_value(mir, value)?);
    }
    Ok(mir_values)
}

fn transform_number(number: HirNumber) -> Result<MirValue> {
    Ok(MirValue::Number(MirNumber {
        value: number.value,
    }))
}

fn transform_bool(bool: HirBool) -> Result<MirValue> {
    Ok(MirValue::Bool(MirBool { value: bool.value }))
}

fn transform_address(
    mir: &mut Mir,
    quote: Quote,
    HirBitAddress { char, ptr, bit }: HirBitAddress,
) -> Result<MirValue> {
    match char {
        b'I' | b'E' => Ok(MirValue::Address(MirAddress {
            r#type: MirAddressType::PhysicalInput1,
            ptr,
            bit,
        })),
        b'Q' | b'A' => Ok(MirValue::Address(MirAddress {
            r#type: MirAddressType::PhysicalOutput1,
            ptr,
            bit,
        })),
        _ => Err(Error::new(
            mir.source.clone(),
            quote,
            Reason::UnknownBitAddressType,
        )),
    }
}

fn transform_not(mir: &mut Mir, not: HirNot) -> Result<MirValue> {
    Ok(MirValue::Not(Rc::new(MirNot {
        value: transform_value(mir, not.value)?,
    })))
}

fn transform_and(mir: &mut Mir, and: HirAnd) -> Result<MirValue> {
    Ok(MirValue::And(Rc::new(MirAnd {
        left: transform_value(mir, and.left)?,
        right: transform_value(mir, and.right)?,
    })))
}

fn transform_or(mir: &mut Mir, or: HirOr) -> Result<MirValue> {
    Ok(MirValue::Or(Rc::new(MirOr {
        left: transform_value(mir, or.left)?,
        right: transform_value(mir, or.right)?,
    })))
}

fn transform_xor(mir: &mut Mir, xor: HirXor) -> Result<MirValue> {
    Ok(MirValue::Xor(Rc::new(MirXor {
        left: transform_value(mir, xor.left)?,
        right: transform_value(mir, xor.right)?,
    })))
}

fn transform_var_ref(mir: &mut Mir, quote: Quote) -> Result<MirValue> {
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

fn transform_call(mir: &mut Mir, quote: Quote, call: HirCall) -> Result<MirValue> {
    let function_name = &mir.source[&call.name];
    if let Some(func) = BUILTIN_FUNCTIONS.get(function_name) {
        let args = transform_values(mir, call.args)?;
        return func(mir, quote, &args);
    };
    Err(Error::new(
        mir.source.clone(),
        call.name,
        Reason::UnknownFunction,
    ))
}

pub(super) fn transform_value(mir: &mut Mir, value: HirValue) -> Result<MirValue> {
    match value.r#type {
        HirValueType::Number(number) => transform_number(number),
        HirValueType::Bool(bool) => transform_bool(bool),
        HirValueType::BitAddress(address) => transform_address(mir, value.quote, address),
        HirValueType::Not(not) => transform_not(mir, *not),
        HirValueType::And(and) => transform_and(mir, *and),
        HirValueType::Or(or) => transform_or(mir, *or),
        HirValueType::Xor(xor) => transform_xor(mir, *xor),
        HirValueType::VarRef(_) => transform_var_ref(mir, value.quote),
        HirValueType::Call(call) => transform_call(mir, value.quote, call),
    }
}
