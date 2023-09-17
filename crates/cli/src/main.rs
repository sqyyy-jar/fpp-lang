use std::{
    fs::read_to_string,
    io::{stdout, IsTerminal},
    path::PathBuf,
    process::exit,
    rc::Rc,
};

use clap::{arg, command, value_parser, Command};
use fpp_compiler::{
    lir::s7::{self, WriteAwl},
    mir,
    parser::Parser,
    util::Source,
};
use messages::message::{Message, MessageContent};

fn main() {
    let terminal = stdout().is_terminal();
    fpp_compiler::init(terminal);
    let matches = command!()
        .subcommand(
            Command::new("awl")
                .about("Compile F++ to AWL")
                .arg(arg!(<FILE> "F++ source file").value_parser(value_parser!(PathBuf))),
        )
        .subcommand_required(true)
        .get_matches();
    match matches.subcommand() {
        Some(("awl", args)) => {
            let file = args.get_one::<PathBuf>("FILE").unwrap();
            let input = read_to_string(file);
            if let Err(err) = input {
                let err = err.to_string();
                let message = Message::error(MessageContent::None, &err);
                eprintln!("{message}");
                exit(1);
            }
            let source = Rc::new(Source::new(file.to_string_lossy(), input.unwrap()));
            let mut parser = Parser::new(source);
            let hir = parser.parse().unwrap_or_else(|err| {
                eprintln!("{err}");
                exit(1);
            });
            let mir = mir::transformer::transform(hir).unwrap_or_else(|err| {
                eprintln!("{err}");
                exit(1);
            });
            let lir = s7::transformer::transform(&mir).expect("S7-LIR");
            lir.networks[0].write_awl(&mut stdout()).expect("Write AWL");
        }
        _ => unreachable!("Unknown subcommand"),
    }
}
