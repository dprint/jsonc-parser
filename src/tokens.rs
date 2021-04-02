use super::common::{ImmutableString, Range, Ranged};

/// A token found while scanning.
#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Comma,
    Colon,
    String(ImmutableString),
    Word(ImmutableString),
    Boolean(bool),
    Number(ImmutableString),
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
