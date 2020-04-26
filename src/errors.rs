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
    pub(super) fn new(range: Range, message: &str) -> ParseError {
        ParseError {
            range,
            message: String::from(message),
        }
    }

    /// Gets the message with a leading range.
    pub fn get_message_with_range(&self, file_text: &str) -> String {
        return format!(
            "{} on line {} column {}.",
            self.message,
            self.range.start_line + 1,
            get_column_number(self.range.start, file_text) + 1,
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
}
