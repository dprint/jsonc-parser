use std::error::Error;
use std::fmt;

use super::common::{Range};

/// Error that could occur while parsing or tokenizing.
#[derive(Debug, PartialEq)]
pub struct ParseError {
    /// Start and end position of the error.
    pub range: Range,
    /// Error message.
    pub message: String,
}

impl ParseError {
    pub(crate) fn new(range: Range, message: &str, file_text: &str) -> ParseError {
        let message = get_message_with_range(&range, message, file_text);
        ParseError {
            message,
            range,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        &self.message
    }
}

fn get_message_with_range(range: &Range, message: &str, file_text: &str) -> String {
    return format!(
        "{} on line {} column {}.",
        message,
        range.start_line + 1,
        get_column_number(range.start, file_text) + 1,
    );

    fn get_column_number(pos: usize, file_text: &str) -> usize {
        let mut column_number = 0;
        for (indice, c) in file_text.char_indices() {
            if c == '\n' {
                column_number = 0;
            } else if indice >= pos {
                break;
            } else {
                column_number += 1;
            }
        }
        column_number
    }
}
