use std::rc::Rc;

use crate::{
    error::Result,
    mir::{
        builtin::assertions::{assert_args_len, assert_readable},
        ops::MirOp,
        value::{MirBitAddress, MirBitAddressType, MirOps, MirValue},
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
///
/// **Idea:**
/// ```rs
/// output = rs(in1, in2);
/// Ops[ResetBit[in2, m0], SetBit[in1, m0], And[m0]]
/// ```
pub fn builtin_rs(mir: &mut Mir, quote: Quote, args: &[MirValue]) -> Result<MirValue> {
    assert_args_len(mir, &quote, args, 2)?;
    assert_readable(mir, &quote, &args[0])?;
    assert_readable(mir, &quote, &args[1])?;
    let (x, y) = mir.memory.alloc_bit().expect("Allocate bit");
    let addr = MirBitAddress {
        r#type: MirBitAddressType::Memory,
        x,
        y,
    };
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
