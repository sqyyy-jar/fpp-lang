use std::fmt::Display;

use crate::{
    util::{find_line, line_number, mark_blue, pad, pad_styled, write_header, write_message},
    yansi::{Color, Style},
};

#[derive(Clone, Copy)]
pub enum MessageType {
    Note,
    Success,
    Warning,
    Error,
}

impl MessageType {
    pub fn color(self) -> Color {
        match self {
            Self::Note => Color::Blue,
            Self::Success => Color::Green,
            Self::Warning => Color::Yellow,
            Self::Error => Color::Red,
        }
    }

    pub fn style(self) -> Style {
        self.color().style().bold()
    }

    pub fn text(self) -> &'static str {
        match self {
            Self::Note => "note",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

pub struct Message<'src> {
    r#type: MessageType,
    content: MessageContent<'src>,
    message: &'src str,
}

impl<'src> Message<'src> {
    pub fn note(content: MessageContent<'src>, message: &'src str) -> Self {
        Self {
            r#type: MessageType::Note,
            content,
            message,
        }
    }

    pub fn success(content: MessageContent<'src>, message: &'src str) -> Self {
        Self {
            r#type: MessageType::Success,
            content,
            message,
        }
    }

    pub fn warning(content: MessageContent<'src>, message: &'src str) -> Self {
        Self {
            r#type: MessageType::Warning,
            content,
            message,
        }
    }

    pub fn error(content: MessageContent<'src>, message: &'src str) -> Self {
        Self {
            r#type: MessageType::Error,
            content,
            message,
        }
    }
}

impl<'src> Display for Message<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.content {
            MessageContent::None => write_message(f, self.r#type, self.message),
            MessageContent::SingleLine {
                file,
                row,
                start_col,
                end_col,
                line,
            } => {
                write_message(f, self.r#type, self.message)?;
                // ---
                let row = row.to_string();
                write_header(f, file, &row, start_col, row.len())?;
                // ---
                pad(f, row.len(), ' ')?;
                writeln!(f, " {}", mark_blue('|'))?;
                // ---
                f.write_str(&row)?;
                writeln!(f, " {} {line}", mark_blue('|'))?;
                // ---
                pad(f, row.len(), ' ')?;
                write!(f, " {} ", mark_blue('|'))?;
                pad(f, start_col as usize, ' ')?;
                pad_styled(f, (end_col - start_col) as usize, '^', self.r#type.style())
            }
            MessageContent::MultiLine {
                file,
                start_row,
                start_col,
                start_line,
                end_row,
                end_col,
                end_line,
            } => {
                write_message(f, self.r#type, self.message)?;
                // ---
                let start_row = start_row.to_string();
                let end_row = end_row.to_string();
                let padding = start_row.len().max(end_row.len());
                write_header(f, file, &start_row, start_col, padding)?;
                // ---
                pad(f, padding, ' ')?;
                writeln!(f, " {}", mark_blue('|'))?;
                // ---
                pad(f, padding - start_row.len(), ' ')?;
                writeln!(
                    f,
                    "{} {} {start_line}",
                    mark_blue(start_row),
                    mark_blue('|')
                )?;
                // ---
                pad(f, padding, ' ')?;
                write!(f, " {} ", mark_blue('|'))?;
                pad(f, start_col as usize, ' ')?;
                pad_styled(
                    f,
                    start_line.len() - start_col as usize,
                    '^',
                    self.r#type.style(),
                )?;
                writeln!(f)?;
                // ---
                pad(f, padding, ' ')?;
                writeln!(f, " {}", mark_blue('.'))?;
                // ---
                pad(f, padding - end_row.len(), ' ')?;
                writeln!(f, "{} {} {end_line}", mark_blue(end_row), mark_blue('|'))?;
                // ---
                pad(f, padding, ' ')?;
                write!(f, " {} ", mark_blue('|'))?;
                pad_styled(f, end_col as usize, '^', self.r#type.style())
            }
        }
    }
}

pub enum MessageContent<'src> {
    None,
    SingleLine {
        file: &'src str,
        row: u32,
        start_col: u32,
        end_col: u32,
        line: &'src str,
    },
    MultiLine {
        file: &'src str,
        start_row: u32,
        start_col: u32,
        start_line: &'src str,
        end_row: u32,
        end_col: u32,
        end_line: &'src str,
    },
}

impl<'src> MessageContent<'src> {
    pub fn parse(file: &'src str, src: &'src str, start: usize, end: usize) -> Option<Self> {
        let (start_row_index, start_row) = line_number(src, start);
        let (end_row_index, end_row) = line_number(src, end - 1);
        if start_row == end_row {
            let line = find_line(src, start_row)?;
            return Some(Self::SingleLine {
                file,
                row: start_row,
                start_col: (start - start_row_index) as u32,
                end_col: (end - end_row_index) as u32,
                line,
            });
        }
        let start_line = find_line(src, start_row)?;
        let end_line = find_line(src, end_row)?;
        Some(Self::MultiLine {
            file,
            start_row,
            start_col: (start - start_row_index) as u32,
            start_line,
            end_row,
            end_col: (end - end_row_index) as u32,
            end_line,
        })
    }
}
