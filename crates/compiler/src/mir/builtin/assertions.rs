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

pub fn assert_readable(mir: &Mir, quote: &Quote, value: &MirValue) -> Result<()> {
    match value {
        MirValue::Bool(_) | MirValue::Number(_) | MirValue::BitAddress(_) | MirValue::Ops(_) => {
            Ok(())
        }
        MirValue::Object(_) => Err(Error::new(
            mir.source.clone(),
            quote.clone(),
            Reason::InvalidArgType,
        )),
        MirValue::Variable(var) => assert_readable(mir, quote, &mir.variables[var.index].value),
        MirValue::Not(not) => assert_readable(mir, quote, &not.value),
        MirValue::And(and) => {
            assert_readable(mir, quote, &and.left)?;
            assert_readable(mir, quote, &and.right)
        }
        MirValue::Or(or) => {
            assert_readable(mir, quote, &or.left)?;
            assert_readable(mir, quote, &or.right)
        }
        MirValue::Xor(xor) => {
            assert_readable(mir, quote, &xor.left)?;
            assert_readable(mir, quote, &xor.right)
        }
    }
}
