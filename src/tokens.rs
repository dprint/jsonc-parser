use super::common::{ Range, Ranged};
use smol_str::SmolStr;
/// A token found while scanning.
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Comma,
    Colon,
    String(SmolStr),
    Word(SmolStr),
    Boolean(bool),
    Number(SmolStr),
    Null,
    CommentLine(SmolStr),
    CommentBlock(SmolStr),
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
