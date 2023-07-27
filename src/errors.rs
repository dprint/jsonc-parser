use std::error::Error;
use std::fmt;

use super::common::Range;

/// Error that could occur while parsing or tokenizing.
#[derive(Debug, PartialEq)]
pub struct ParseError {
  /// Start and end position of the error.
  pub range: Range,
  /// Error message.
  pub message: String,
  /// Message with the range text.
  display_message: String,
}

impl ParseError {
  pub(crate) fn new(range: Range, message: &str, file_text: &str) -> ParseError {
    let display_message = get_message_with_range(&range, message, file_text);
    ParseError {
      message: message.to_string(),
      range,
      display_message,
    }
  }
}

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.display_message)
  }
}

impl Error for ParseError {
  fn description(&self) -> &str {
    &self.display_message
  }
}

fn get_message_with_range(range: &Range, message: &str, file_text: &str) -> String {
  let mut line_index = 0;
  let mut column_index = 0;
  for c in file_text[..range.start].chars() {
    if c == '\n' {
      line_index += 1;
      column_index = 0;
    } else {
      column_index += 1;
    }
  }
  format!("{} on line {} column {}.", message, line_index + 1, column_index + 1,)
}
