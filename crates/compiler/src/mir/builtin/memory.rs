use crate::{
    error::Result,
    mir::{
        value::{MirAddress, MirAddressType, MirValue},
        Mir,
    },
    util::Quote,
};

use super::assertions::assert_args_len;

/// # Allocate memory byte
pub fn builtin_mb(mir: &mut Mir, quote: Quote, args: &[MirValue]) -> Result<MirValue> {
    assert_args_len(mir, &quote, args, 0)?;
    let ptr = mir.memory.alloc_u8().expect("Allocate u8");
    Ok(MirValue::Address(MirAddress {
        r#type: MirAddressType::Memory8,
        ptr,
    }))
}

/// # Allocate memory word
pub fn builtin_mw(mir: &mut Mir, quote: Quote, args: &[MirValue]) -> Result<MirValue> {
    assert_args_len(mir, &quote, args, 0)?;
    let ptr = mir.memory.alloc_u16().expect("Allocate u16");
    Ok(MirValue::Address(MirAddress {
        r#type: MirAddressType::Memory16,
        ptr,
    }))
}

/// # Allocate memory double word
pub fn builtin_md(mir: &mut Mir, quote: Quote, args: &[MirValue]) -> Result<MirValue> {
    assert_args_len(mir, &quote, args, 0)?;
    let ptr = mir.memory.alloc_u32().expect("Allocate u32");
    Ok(MirValue::Address(MirAddress {
        r#type: MirAddressType::Memory32,
        ptr,
    }))
}
