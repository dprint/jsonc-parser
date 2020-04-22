use super::common::ImmutableString;

/// A JSON value.
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    StringLit(StringLit),
    NumberLit(NumberLit),
    BooleanLit(BooleanLit),
    Object(Object),
    Array(Array),
    NullKeyword(NullKeyword),
}

/// Information about where the node appears in the text file.
#[derive(Debug, PartialEq, Clone)]
pub struct Range {
    /// Start position of the node in the file text.
    pub pos: usize,
    /// End position of the node in the file text.
    pub end: usize,
    /// Line of the start position of the node in the file text.
    pub start_line: usize,
    /// Line of the end position of the node in the file text.
    pub end_line: usize,
}

/// Node surrounded in double quotes (ex. `"my string"`).
#[derive(Debug, PartialEq, Clone)]
pub struct StringLit {
    pub range: Range,
    pub value: ImmutableString,
}

/// Node that represents a number (ex. `123`, `99.99`, `-1.2e+2`).
#[derive(Debug, PartialEq, Clone)]
pub struct NumberLit {
    pub range: Range,
    pub value: ImmutableString,
}

/// Node that represents a boolean (ex. `true` or `false`).
#[derive(Debug, PartialEq, Clone)]
pub struct BooleanLit {
    pub range: Range,
    pub value: bool,
}

/// Node that represents the null keyword (ex. `null`).
#[derive(Debug, PartialEq, Clone)]
pub struct NullKeyword {
    pub range: Range,
}

/// Node that represents an object that may contain properties (ex. `{}`, `{ "prop": 4 }`).
#[derive(Debug, PartialEq, Clone)]
pub struct Object {
    pub range: Range,
    pub properties: Vec<ObjectProp>,
}

/// Node that represents an object property (ex. `"prop": []`).
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectProp {
    pub range: Range,
    pub name: StringLit,
    pub value: Value,
}

/// Node that represents an array that may contain elements (ex. `[]` or `[5, 6]`).
#[derive(Debug, PartialEq, Clone)]
pub struct Array {
    pub range: Range,
    pub elements: Vec<Value>,
}

/// Represents a comment.
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
