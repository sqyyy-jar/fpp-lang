use fpp_compiler::parser::{Parser, ParserOptions};

const SRC: &[u8] = br#"
let button = in(2.4);
let led = out(1.2);

led = button and not led;
"#;

fn main() {
    let mut parser = Parser::new(
        SRC.into(),
        ParserOptions {
            ..Default::default()
        },
    );
    let res = parser.parse();
    println!("{res:#?}");
}
