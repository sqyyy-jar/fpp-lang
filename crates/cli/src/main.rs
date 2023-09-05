use fpp_compiler::parser::Parser;

const SRC: &[u8] = br#"
let pos_auf = E0.5;
let t_stop = E0.1;
let t_auf = E0.2;
let tor_schliessen = A4.1;
let sr1 = sr(t_auf, pos_auf & t_stop & tor_schliessen, tor_schliessen);

tor_schliessen = value(sr1);
"#;

fn main() {
    let mut parser = Parser::new(SRC.into());
    let res = parser.parse();
    println!("{res:#?}");
}
