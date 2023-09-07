use crate::{
    error::{Error, Reason, Result},
    mir::{value::MirValue, Mir},
    util::Quote,
};

pub fn assert_args_len(mir: &Mir, quote: &Quote, args: &[MirValue], len: usize) -> Result<()> {
    if args.len() != len {
        return Err(Error::new(
            mir.source.clone(),
            quote.clone(),
            Reason::InvalidArgsCount,
        ));
    }
    Ok(())
}

pub fn assert_bit_readable(mir: &Mir, quote: &Quote, value: &MirValue) -> Result<()> {
    if !value.is_bit_readable(mir) {
        return Err(Error::new(
            mir.source.clone(),
            quote.clone(),
            Reason::InvalidArgType,
        ));
    }
    Ok(())
}
