use crate::{
    error::Result,
    mir::{value::MirValue, Mir},
    util::Quote,
};

use super::assertions::assert_args_len;

/// Allocate memory bit
pub fn builtin_m(mir: &mut Mir, quote: Quote, args: &[MirValue]) -> Result<MirValue> {
    assert_args_len(mir, &quote, args, 0)?;
    let addr = mir.memory.alloc1().expect("Allocate u1");
    Ok(MirValue::Address(addr))
}

/// Allocate memory byte
pub fn builtin_mb(mir: &mut Mir, quote: Quote, args: &[MirValue]) -> Result<MirValue> {
    assert_args_len(mir, &quote, args, 0)?;
    let addr = mir.memory.alloc8().expect("Allocate u8");
    Ok(MirValue::Address(addr))
}

/// Allocate memory word
pub fn builtin_mw(mir: &mut Mir, quote: Quote, args: &[MirValue]) -> Result<MirValue> {
    assert_args_len(mir, &quote, args, 0)?;
    let addr = mir.memory.alloc16().expect("Allocate u16");
    Ok(MirValue::Address(addr))
}

/// Allocate memory double word
pub fn builtin_md(mir: &mut Mir, quote: Quote, args: &[MirValue]) -> Result<MirValue> {
    assert_args_len(mir, &quote, args, 0)?;
    let addr = mir.memory.alloc32().expect("Allocate u32");
    Ok(MirValue::Address(addr))
}
