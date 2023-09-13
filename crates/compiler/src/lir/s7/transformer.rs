use crate::{
    lir::error::{Error, Result},
    mir::{
        value::{MirAddress, MirAddressType},
        Mir, MirAction, MirInstruction,
    },
};

use super::{S7Address, S7AddressType, S7Instruction, S7Lir, S7Network};

fn push_stack(stack_depth: &mut usize) -> Result<()> {
    if *stack_depth >= 7 {
        return Err(Error::InvalidStack);
    }
    *stack_depth += 1;
    Ok(())
}

fn pop_stack(stack_depth: &mut usize) -> Result<()> {
    if *stack_depth == 0 {
        return Err(Error::InvalidStack);
    }
    *stack_depth -= 1;
    Ok(())
}

fn transform_address(mir: &Mir, addr: MirAddress) -> S7Address {
    let r#type = match addr.r#type {
        MirAddressType::PhysicalInput1 => S7AddressType::Input1,
        MirAddressType::PhysicalOutput1 => S7AddressType::Output1,
        MirAddressType::PhysicalCounter => S7AddressType::Counter,
        MirAddressType::Memory1 => S7AddressType::Memory1,
        MirAddressType::Memory8 => S7AddressType::Memory8,
        MirAddressType::Memory16 => S7AddressType::Memory16,
        MirAddressType::Memory32 => S7AddressType::Memory32,
    };
    let ptr = if addr.is_virtual() {
        mir.allocator.byte_offset() as u16 + addr.ptr
    } else {
        addr.ptr
    };
    let bit = addr.bit;
    S7Address { r#type, ptr, bit }
}

fn assert_bit(mir: &Mir, addr: MirAddress) -> Result<S7Address> {
    if !addr.is_bit_address() {
        return Err(Error::NonBitAddress);
    }
    Ok(transform_address(mir, addr))
}

fn assert_any_byte(mir: &Mir, addr: MirAddress) -> Result<S7Address> {
    if !addr.is_any_byte_address() {
        return Err(Error::NonBitAddress);
    }
    Ok(transform_address(mir, addr))
}

fn assert_counter(mir: &Mir, addr: MirAddress) -> Result<S7Address> {
    if addr.r#type != MirAddressType::PhysicalCounter {
        return Err(Error::NonCounterAddress);
    }
    Ok(transform_address(mir, addr))
}

fn transform_instructions(
    mir: &Mir,
    src: &[MirInstruction],
    dst: &mut Vec<S7Instruction>,
) -> Result<()> {
    let mut stack_depth = 0;
    for &instruction in src {
        match instruction {
            MirInstruction::Dummy => {}
            MirInstruction::And { addr } => {
                let addr = assert_bit(mir, addr)?;
                dst.push(S7Instruction::And { addr });
            }
            MirInstruction::AndNot { addr } => {
                let addr = assert_bit(mir, addr)?;
                dst.push(S7Instruction::AndNot { addr });
            }
            MirInstruction::Or { addr } => {
                let addr = assert_bit(mir, addr)?;
                dst.push(S7Instruction::Or { addr });
            }
            MirInstruction::OrNot { addr } => {
                let addr = assert_bit(mir, addr)?;
                dst.push(S7Instruction::OrNot { addr });
            }
            MirInstruction::Xor { addr } => {
                let addr = assert_bit(mir, addr)?;
                dst.push(S7Instruction::Xor { addr });
            }
            MirInstruction::XorNot { addr } => {
                let addr = assert_bit(mir, addr)?;
                dst.push(S7Instruction::XorNot { addr });
            }
            MirInstruction::AndStart => {
                push_stack(&mut stack_depth)?;
                dst.push(S7Instruction::AndStart);
            }
            MirInstruction::AndNotStart => {
                push_stack(&mut stack_depth)?;
                dst.push(S7Instruction::AndNotStart);
            }
            MirInstruction::OrStart => {
                push_stack(&mut stack_depth)?;
                dst.push(S7Instruction::OrStart);
            }
            MirInstruction::OrNotStart => {
                push_stack(&mut stack_depth)?;
                dst.push(S7Instruction::OrNotStart);
            }
            MirInstruction::XorStart => {
                push_stack(&mut stack_depth)?;
                dst.push(S7Instruction::XorStart);
            }
            MirInstruction::XorNotStart => {
                push_stack(&mut stack_depth)?;
                dst.push(S7Instruction::XorNotStart);
            }
            MirInstruction::End => {
                pop_stack(&mut stack_depth)?;
                dst.push(S7Instruction::End);
            }
            MirInstruction::AssignBit { addr } => {
                let addr = assert_bit(mir, addr)?;
                dst.push(S7Instruction::AssignBit { addr });
            }
            MirInstruction::ResetBit { addr } => {
                let addr = assert_bit(mir, addr)?;
                dst.push(S7Instruction::ResetBit { addr });
            }
            MirInstruction::SetBit { addr } => {
                let addr = assert_bit(mir, addr)?;
                dst.push(S7Instruction::SetBit { addr });
            }
            MirInstruction::Not => dst.push(S7Instruction::Not),
            MirInstruction::Set => dst.push(S7Instruction::Set),
            MirInstruction::Clear => dst.push(S7Instruction::Clear),
            MirInstruction::Save => dst.push(S7Instruction::Save),
            MirInstruction::CounterLoadInt { addr } => {
                let addr = assert_counter(mir, addr)?;
                dst.push(S7Instruction::CounterLoadInt { addr });
            }
            MirInstruction::CounterLoadBcd { addr } => {
                let addr = assert_counter(mir, addr)?;
                dst.push(S7Instruction::CounterLoadBcd { addr });
            }
            MirInstruction::CounterReset { addr } => {
                let addr = assert_counter(mir, addr)?;
                dst.push(S7Instruction::CounterReset { addr });
            }
            MirInstruction::CounterSet { addr } => {
                let addr = assert_counter(mir, addr)?;
                dst.push(S7Instruction::CounterSet { addr });
            }
            MirInstruction::CounterForward { addr } => {
                let addr = assert_counter(mir, addr)?;
                dst.push(S7Instruction::CounterForward { addr });
            }
            MirInstruction::CounterBackward { addr } => {
                let addr = assert_counter(mir, addr)?;
                dst.push(S7Instruction::CounterBackward { addr });
            }
            MirInstruction::Transfer { addr } => {
                let addr = assert_any_byte(mir, addr)?;
                dst.push(S7Instruction::Transfer { addr });
            }
        }
    }
    if stack_depth != 0 {
        return Err(Error::InvalidStack);
    }
    Ok(())
}

fn transform_action(lir: &mut S7Lir, mir: &Mir, action: &MirAction) -> Result<()> {
    let network = &mut lir.networks[0];
    // Clear `/ER`
    network.instructions.push(S7Instruction::Clear);
    match action {
        MirAction::Raw(raw) => {
            transform_instructions(mir, &raw.instructions, &mut network.instructions)?;
        }
        MirAction::Output(output) => {
            let addr = assert_bit(mir, output.address)?;
            transform_instructions(mir, &output.instructions, &mut network.instructions)?;
            network.instructions.push(S7Instruction::AssignBit { addr });
        }
    }
    Ok(())
}

pub fn transform(mir: &Mir) -> Result<S7Lir> {
    let mut lir = S7Lir::default();
    lir.networks.push(S7Network::default());
    for action in &mir.actions {
        transform_action(&mut lir, mir, action)?;
    }
    Ok(lir)
}
