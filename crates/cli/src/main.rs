use fpp_compiler::{compiler::compile, parser::Parser};

const SRC: &[u8] = br#"
let pos_auf = E0.5;
let t_stop = E0.1;
let t_auf = E0.2;
let tor_schliessen = A4.1;

tor_schliessen = sr(t_auf, pos_auf & t_stop & tor_schliessen);
"#;

fn main() {
    let mut parser = Parser::new(SRC.into());
    let hir = parser.parse().expect("HIR");
    println!("{hir:#?}");
    let mir = compile(hir);
    println!("{mir:#?}");
}
