pub mod error;
pub mod hir;
pub mod lir;
pub mod mir;
pub mod parser;
pub mod util;

pub fn init(terminal: bool) {
    if !terminal {
        messages::yansi::Paint::disable();
    }
}
