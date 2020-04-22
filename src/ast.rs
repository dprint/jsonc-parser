use super::common::{ImmutableString, Range};

/// Different kinds of JSON values.
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    StringLit(StringLit),
    NumberLit(NumberLit),
    BooleanLit(BooleanLit),
    Object(Object),
    Array(Array),
    NullKeyword(NullKeyword),
}

/// Node surrounded in double quotes (ex. `"my string"`).
#[derive(Debug, PartialEq, Clone)]
pub struct StringLit {
    pub range: Range,
    pub value: ImmutableString,
}

/// Represents a number (ex. `123`, `99.99`, `-1.2e+2`).
#[derive(Debug, PartialEq, Clone)]
pub struct NumberLit {
    pub range: Range,
    pub value: ImmutableString,
}

/// Represents a boolean (ex. `true` or `false`).
#[derive(Debug, PartialEq, Clone)]
pub struct BooleanLit {
    pub range: Range,
    pub value: bool,
}

/// Represents the null keyword (ex. `null`).
#[derive(Debug, PartialEq, Clone)]
pub struct NullKeyword {
    pub range: Range,
}

/// Represents an object that may contain properties (ex. `{}`, `{ "prop": 4 }`).
#[derive(Debug, PartialEq, Clone)]
pub struct Object {
    pub range: Range,
    pub properties: Vec<ObjectProp>,
}

/// Represents an object property (ex. `"prop": []`).
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectProp {
    pub range: Range,
    pub name: StringLit,
    pub value: Value,
}

/// Represents an array that may contain elements (ex. `[]` or `[5, 6]`).
#[derive(Debug, PartialEq, Clone)]
pub struct Array {
    pub range: Range,
    pub elements: Vec<Value>,
}

/// Different kinds of JSONC comments.
#[derive(Debug, PartialEq, Clone)]
pub enum Comment {
    Line(CommentLine),
    Block(CommentBlock)
}

/// Represents a comment line (ex. `// my comment`).
#[derive(Debug, PartialEq, Clone)]
pub struct CommentLine {
    pub range: Range,
    pub text: ImmutableString,
}

/// Represents a comment block (ex. `/* my comment */`).
#[derive(Debug, PartialEq, Clone)]
pub struct CommentBlock {
    pub range: Range,
    pub text: ImmutableString,
}
