use std::{env::args, fs::read_to_string, io::stdout, process::exit};

use fpp_compiler::{
    lir::s7::{self, WriteAwl},
    mir,
    parser::Parser,
};

fn main() {
    let input = read_to_string(args().nth(1).expect("Input file"))
        .expect("Readable input file")
        .into();
    let mut parser = Parser::new(input);
    let hir = parser.parse().unwrap_or_else(|err| {
        eprintln!("{err}");
        exit(1);
    });
    // println!("{hir:#?}");
    let mir = mir::transformer::transform(hir).unwrap_or_else(|err| {
        eprintln!("{err}");
        exit(1);
    });
    // println!("{mir:#?}");
    let lir = s7::transformer::transform(&mir).expect("S7-LIR");
    // println!("{lir:#?}");
    lir.networks[0].write_awl(&mut stdout()).expect("Write AWL");
}
