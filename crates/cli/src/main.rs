use fpp_compiler::{
    lir::s7::{self},
    mir,
    parser::Parser,
};

const SRC: &[u8] = br#"
let pos_auf = I0.5;
let t_stop = I0.1;
let t_auf = I0.2;
let tor_schliessen = Q4.1;

tor_schliessen = sr(t_auf, pos_auf & t_stop & tor_schliessen);
"#;

fn main() {
    let mut parser = Parser::new(SRC.into());
    let hir = parser.parse().expect("HIR");
    println!("{hir:#?}");
    let mir = mir::transformer::transform(hir).expect("MIR");
    println!("{mir:#?}");
    let lir = s7::transformer::transform(&mir).expect("LIR");
    println!("{lir:#?}");
}
