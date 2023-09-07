use std::rc::Rc;

use crate::{
    error::Result,
    mir::{
        builtin::assertions::{assert_args_len, assert_bit_readable},
        ops::MirOp,
        value::{MirOps, MirValue},
        Mir,
    },
    util::Quote,
};

/// # RS-Flipflop
///
/// **Signature:**
/// ```rs
/// fn(r_input, s_input) -> output
/// ```
pub fn builtin_rs(mir: &mut Mir, quote: Quote, args: &[MirValue]) -> Result<MirValue> {
    assert_args_len(mir, &quote, args, 2)?;
    assert_bit_readable(mir, &quote, &args[0])?;
    assert_bit_readable(mir, &quote, &args[1])?;
    let addr = mir.memory.alloc1().expect("Allocate bit");
    let reset_bit = MirOp::ResetBit {
        cond: args[0].clone(),
        addr,
    };
    let set_bit = MirOp::SetBit {
        cond: args[1].clone(),
        addr,
    };
    let and = MirOp::And { addr };
    Ok(MirValue::Ops(Rc::new(MirOps {
        ops: vec![reset_bit, set_bit, and],
    })))
}

/// # SR-Flipflop
///
/// **Signature:**
/// ```rs
/// fn(s_input, r_input) -> output
/// ```
pub fn builtin_sr(mir: &mut Mir, quote: Quote, args: &[MirValue]) -> Result<MirValue> {
    assert_args_len(mir, &quote, args, 2)?;
    assert_bit_readable(mir, &quote, &args[0])?;
    assert_bit_readable(mir, &quote, &args[1])?;
    let addr = mir.memory.alloc1().expect("Allocate bit");
    let set_bit = MirOp::SetBit {
        cond: args[0].clone(),
        addr,
    };
    let reset_bit = MirOp::ResetBit {
        cond: args[1].clone(),
        addr,
    };
    let and = MirOp::And { addr };
    Ok(MirValue::Ops(Rc::new(MirOps {
        ops: vec![set_bit, reset_bit, and],
    })))
}
