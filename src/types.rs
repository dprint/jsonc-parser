#[derive(Debug, PartialEq)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Comma,
    Colon,
    String(String),
    Boolean(bool),
    Number(String),
    Null,
    CommentLine(String),
    CommentBlock(String),
}

pub enum Value {
    StringLit,
    Object,
}

pub struct Range {
    pos: usize,
    end: usize,
    line_start: usize,
    line_end: usize,
}

pub struct StringLit {
    range: Range,
    value: String,
}

pub struct Object {
    range: Range,
    properties: Vec<ObjectProp>,
}

pub struct ObjectProp {
    range: Range,
    name: StringLit,
    value: Value,
}

pub struct Array {
    range: Range,
    elements: Vec<Value>,
}