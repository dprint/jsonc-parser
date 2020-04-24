use super::common::{ImmutableString, Range, Ranged};

/// A token found while scanning.
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Comma,
    Colon,
    String(ImmutableString),
    Boolean(bool),
    Number(ImmutableString),
    Null,
    CommentLine(ImmutableString),
    CommentBlock(ImmutableString),
}

/// A token with positional information.
pub struct TokenAndRange {
    pub range: Range,
    pub token: Token,
}

impl Ranged for TokenAndRange {
    fn range(&self) -> &Range {
        &self.range
    }
}
