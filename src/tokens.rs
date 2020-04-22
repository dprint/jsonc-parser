use super::common::ImmutableString;

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
