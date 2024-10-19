use std::fmt;

use crate::ParseStringErrorKind;

use super::common::Range;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseErrorKind {
  CommentsNotAllowed,
  ExpectedColonAfterObjectKey,
  ExpectedObjectValue,
  ExpectedDigit,
  ExpectedDigitFollowingNegativeSign,
  ExpectedPlusMinusOrDigitInNumberLiteral,
  ExpectedStringObjectProperty,
  MultipleRootJsonValues,
  String(ParseStringErrorKind),
  TrailingCommasNotAllowed,
  UnexpectedCloseBrace,
  UnexpectedCloseBracket,
  UnexpectedColon,
  UnexpectedComma,
  UnexpectedToken,
  UnexpectedTokenInObject,
  UnexpectedWord,
  UnterminatedArray,
  UnterminatedCommentBlock,
  UnterminatedObject,
}

impl std::fmt::Display for ParseErrorKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use ParseErrorKind::*;
    match self {
      CommentsNotAllowed => {
        write!(f, "Comments are not allowed")
      }
      ExpectedColonAfterObjectKey => {
        write!(f, "Expected colon after the string or word in object property")
      }
      ExpectedDigit => {
        write!(f, "Expected digit")
      }
      ExpectedDigitFollowingNegativeSign => {
        write!(f, "Expected digit following negative sign")
      }
      ExpectedPlusMinusOrDigitInNumberLiteral => {
        write!(f, "Expected plus, minus, or digit in number literal")
      }
      ExpectedObjectValue => {
        write!(f, "Expected value after colon in object property")
      }
      ExpectedStringObjectProperty => {
        write!(f, "Expected string for object property")
      }
      MultipleRootJsonValues => {
        write!(f, "Text cannot contain more than one JSON value")
      }
      String(kind) => kind.fmt(f),
      TrailingCommasNotAllowed => {
        write!(f, "Trailing commas are not allowed")
      }
      UnexpectedCloseBrace => {
        write!(f, "Unexpected close brace")
      }
      UnexpectedCloseBracket => {
        write!(f, "Unexpected close bracket")
      }
      UnexpectedColon => {
        write!(f, "Unexpected colon")
      }
      UnexpectedComma => {
        write!(f, "Unexpected comma")
      }
      UnexpectedWord => {
        write!(f, "Unexpected word")
      }
      UnexpectedToken => {
        write!(f, "Unexpected token")
      }
      UnexpectedTokenInObject => {
        write!(f, "Unexpected token in object")
      }
      UnterminatedArray => {
        write!(f, "Unterminated array")
      }
      UnterminatedCommentBlock => {
        write!(f, "Unterminated comment block")
      }
      UnterminatedObject => {
        write!(f, "Unterminated object")
      }
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
struct ParseErrorInner {
  range: Range,
  line_display: usize,
  column_display: usize,
  kind: ParseErrorKind,
}

/// Error that could occur while parsing or tokenizing.
#[derive(Debug, Clone, PartialEq)]
pub struct ParseError(Box<ParseErrorInner>);

impl std::error::Error for ParseError {}

impl ParseError {
  pub(crate) fn new(range: Range, kind: ParseErrorKind, file_text: &str) -> ParseError {
    let (line_display, column_display) = get_line_and_column_display(range, file_text);
    ParseError(Box::new(ParseErrorInner {
      range,
      line_display,
      column_display,
      kind,
    }))
  }

  /// Start and end position of the error.
  pub fn range(&self) -> Range {
    self.0.range
  }

  /// 1-indexed line number the error occurred on.
  pub fn line_display(&self) -> usize {
    self.0.line_display
  }

  /// 1-indexed column number the error occurred on.
  ///
  /// Note: Use the `error_unicode_width` feature to get the correct column
  /// number for Unicode characters on the line, otherwise this is just the
  /// number of characters by default.
  pub fn column_display(&self) -> usize {
    self.0.column_display
  }

  /// Error message.
  pub fn kind(&self) -> &ParseErrorKind {
    &self.0.kind
  }
}

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let inner = &*self.0;
    write!(
      f,
      "{} on line {} column {}",
      inner.kind, inner.line_display, inner.column_display
    )
  }
}

fn get_line_and_column_display(range: Range, file_text: &str) -> (usize, usize) {
  let mut line_index = 0;
  let mut column_index = 0;
  for c in file_text[..range.start].chars() {
    if c == '\n' {
      line_index += 1;
      column_index = 0;
    } else {
      #[cfg(feature = "error_unicode_width")]
      {
        if let Some(width) = unicode_width::UnicodeWidthChar::width_cjk(c) {
          column_index += width;
        }
      }
      #[cfg(not(feature = "error_unicode_width"))]
      {
        column_index += 1;
      }
    }
  }
  (line_index + 1, column_index + 1)
}
