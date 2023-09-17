use std::fmt::Write;

use yansi::Style;

use crate::{
    message::MessageType,
    yansi::{Color, Paint},
};

pub fn line_number(src: &str, start: usize) -> (usize, u32) {
    let mut line = 1;
    let mut line_start = 0;
    for (i, &c) in src.as_bytes().iter().enumerate() {
        if c == b'\n' {
            line_start = i + 1;
            line += 1;
        }
        if i >= start {
            break;
        }
    }
    (line_start, line)
}

pub fn find_line(src: &str, line: u32) -> Option<&str> {
    let mut current_line = 1;
    let mut start_index = 0;
    for (i, &c) in src.as_bytes().iter().enumerate() {
        if current_line == line {
            start_index = i;
            break;
        }
        if c == b'\n' {
            current_line += 1;
        }
    }
    let src = &src[start_index..];
    for (i, &c) in src.as_bytes().iter().enumerate() {
        if c == b'\n' {
            return Some(&src[..i]);
        }
    }
    None
}

pub fn write_message(
    out: &mut std::fmt::Formatter,
    r#type: MessageType,
    message: &str,
) -> std::fmt::Result {
    let style = r#type.style();
    writeln!(
        out,
        "{}{} {}",
        style.paint(r#type.text()),
        style.paint(':').fg(Color::Default),
        message
    )
}

pub fn write_header(
    out: &mut std::fmt::Formatter,
    file: &str,
    row: &str,
    start_col: u32,
    padding: usize,
) -> std::fmt::Result {
    pad(out, padding, ' ')?;
    writeln!(out, "{} {file}:{row}:{}", mark_blue("-->"), start_col + 1)
}

pub fn pad(out: &mut std::fmt::Formatter, amount: usize, c: char) -> std::fmt::Result {
    for _ in 0..amount {
        out.write_char(c)?;
    }
    Ok(())
}

pub fn pad_styled(
    out: &mut std::fmt::Formatter,
    amount: usize,
    c: char,
    style: Style,
) -> std::fmt::Result {
    for _ in 0..amount {
        write!(out, "{}", style.paint(c))?;
    }
    Ok(())
}

pub fn mark_blue<T>(value: T) -> Paint<T> {
    Paint::new(value).fg(Color::Blue).bold()
}
