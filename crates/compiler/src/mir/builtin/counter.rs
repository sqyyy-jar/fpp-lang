use crate::{
    error::Result,
    mir::{value::MirValue, Mir},
    util::Quote,
};

use super::assertions::assert_args_len;

/// Allocate counter
pub fn builtin_counter(mir: &mut Mir, quote: Quote, args: &[MirValue]) -> Result<MirValue> {
    assert_args_len(mir, &quote, args, 0)?;
    let addr = mir.allocator.alloc_counter().expect("Allocate counter");
    Ok(MirValue::Address(addr))
}
