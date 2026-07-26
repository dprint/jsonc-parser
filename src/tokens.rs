use super::common::Range;
use super::common::Ranged;
use std::borrow::Cow;

/// A token found while scanning.
#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
  OpenBrace,
  CloseBrace,
  OpenBracket,
  CloseBracket,
  Comma,
  Colon,
  String(Cow<'a, str>),
  Word(&'a str),
  Boolean(bool),
  Number(&'a str),
  Null,
  CommentLine(&'a str),
  CommentBlock(&'a str),
}

impl<'a> Token<'a> {
  pub fn as_str(&self) -> &str {
    match self {
      Token::OpenBrace => "{",
      Token::CloseBrace => "}",
      Token::OpenBracket => "[",
      Token::CloseBracket => "]",
      Token::Comma => ",",
      Token::Colon => ":",
      Token::String(value) => value,
      Token::Word(value) => value,
      Token::Boolean(value) => {
        if *value {
          "true"
        } else {
          "false"
        }
      }
      Token::Number(value) => value,
      Token::Null => "null",
      Token::CommentLine(value) => value,
      Token::CommentBlock(value) => value,
    }
  }

  /// Whether this token can begin a JSON value.
  pub(crate) fn is_value_start(&self) -> bool {
    matches!(
      self,
      Token::OpenBrace | Token::OpenBracket | Token::String(_) | Token::Boolean(_) | Token::Number(_) | Token::Null
    )
  }
}

/// A token with positional information.
pub struct TokenAndRange<'a> {
  pub range: Range,
  pub token: Token<'a>,
}

impl<'a> Ranged for TokenAndRange<'a> {
  fn range(&self) -> Range {
    self.range
  }
}
