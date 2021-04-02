use super::common::{Range, Ranged};
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

/// A token with positional information.
pub struct TokenAndRange<'a> {
    pub range: Range,
    pub token: Token<'a>,
}

impl<'a> Ranged for TokenAndRange<'a> {
    fn range(&self) -> &Range {
        &self.range
    }
}
