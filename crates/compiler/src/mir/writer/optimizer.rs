use crate::mir::MirInstruction;

use super::MirInstructionWriter;

fn non_dummy_len(slice: &[MirInstruction]) -> usize {
    slice.iter().filter(|&it| !it.is_dummy()).count()
}

fn get_non_dummy(slice: &[MirInstruction], index: usize) -> &MirInstruction {
    slice
        .iter()
        .filter(|it| !it.is_dummy())
        .nth(index)
        .expect("Not enough elements")
}

fn fill_dummy(slice: &mut [MirInstruction]) {
    for instruction in slice {
        *instruction = MirInstruction::Dummy;
    }
}

fn optimize_brackets(slice: &mut [MirInstruction], mut index: usize, bracket_term: bool) -> usize {
    while index < slice.len() {
        let start = slice[index];
        if start.is_dummy() {
            index += 1;
            continue;
        }
        if start.is_end() {
            assert!(bracket_term, "Invalid bytecode");
            return index;
        }
        if !start.has_bracket() {
            index += 1;
            continue;
        }
        let term_end = optimize_brackets(slice, index + 1, true);
        let term = &mut slice[index + 1..term_end];
        let len = non_dummy_len(term);
        if len < 1 {
            panic!("Invalid bytecode: {term:?}");
        }
        if len > 1 {
            index = term_end + 1;
            continue;
        }
        let inner = *get_non_dummy(term, 0);
        if !matches!(inner, MirInstruction::And { .. }) {
            continue;
        }
        fill_dummy(term);
        slice[term_end] = MirInstruction::Dummy;
        slice[index] = start.unbracket(inner.addr());
        index = term_end + 1;
    }
    assert!(!bracket_term, "Invalid bytecode");
    index
}

fn filter_dummy(writer: &mut MirInstructionWriter) {
    writer.instructions = std::mem::replace(&mut writer.instructions, Vec::with_capacity(0))
        .into_iter()
        .filter(|it| !it.is_dummy())
        .collect();
}

pub fn optimize(writer: &mut MirInstructionWriter) {
    optimize_brackets(&mut writer.instructions, 0, false);
    filter_dummy(writer);
}
