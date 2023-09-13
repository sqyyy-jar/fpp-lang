use crate::{
    error::Result,
    mir::{value::MirValue, Mir},
    util::Quote,
};

use super::assertions::assert_args_len;

/// Allocate memory bit
pub fn builtin_counter(mir: &mut Mir, quote: Quote, args: &[MirValue]) -> Result<MirValue> {
    assert_args_len(mir, &quote, args, 0)?;
    todo!()
}
