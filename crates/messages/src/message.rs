use std::fmt::Display;

use yansi::{Color, Style};

use crate::util::{find_line, line_number, pad, write_header, write_message};

#[derive(Clone, Copy)]
pub enum MessageType {
    Note,
    Success,
    Warning,
    Error,
}

impl MessageType {
    pub fn style(self) -> Style {
        match self {
            Self::Note => Style::new(Color::Blue).bold(),
            Self::Success => Style::new(Color::Green).bold(),
            Self::Warning => Style::new(Color::Yellow).bold(),
            Self::Error => Style::new(Color::Red).bold(),
        }
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
                let row = row.to_string();
                write_header(f, file, &row, start_col, row.len())?;
                // ---
                pad(f, row.len(), ' ')?;
                writeln!(f, " |")?;
                // ---
                f.write_str(&row)?;
                writeln!(f, " | {line}")?;
                // ---
                pad(f, row.len(), ' ')?;
                write!(f, " | ")?;
                pad(f, start_col as usize, ' ')?;
                pad(f, (end_col - start_col) as usize, '^')?;
                writeln!(f, "\n")?;
                // ---
                write_message(f, self.r#type, self.message)
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
                let start_row = start_row.to_string();
                let end_row = end_row.to_string();
                let padding = start_row.len().max(end_row.len());
                write_header(f, file, &start_row, start_col, padding)?;
                // ---
                pad(f, padding, ' ')?;
                writeln!(f, " |")?;
                // ---
                pad(f, padding - start_row.len(), ' ')?;
                f.write_str(&start_row)?;
                writeln!(f, " | {start_line}")?;
                // ---
                pad(f, padding, ' ')?;
                write!(f, " | ")?;
                pad(f, start_col as usize, ' ')?;
                pad(f, start_line.len() - start_col as usize, '^')?;
                writeln!(f)?;
                // ---
                pad(f, padding, ' ')?;
                writeln!(f, " .")?;
                // ---
                pad(f, padding - end_row.len(), ' ')?;
                f.write_str(&end_row)?;
                writeln!(f, " | {end_line}")?;
                // ---
                pad(f, padding, ' ')?;
                write!(f, " | ")?;
                pad(f, end_col as usize, '^')?;
                writeln!(f, "\n")?;
                // ---
                write_message(f, self.r#type, self.message)
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
        let (end_row_index, end_row) = line_number(src, end);
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
