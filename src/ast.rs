use super::common::{ImmutableString, Range, Ranged};

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

/// Different nodes that can appear in the AST.
#[derive(Debug, PartialEq, Clone)]
pub enum Node<'a> {
    StringLit(&'a StringLit),
    NumberLit(&'a NumberLit),
    BooleanLit(&'a BooleanLit),
    Object(&'a Object),
    ObjectProp(&'a ObjectProp),
    Array(&'a Array),
    NullKeyword(&'a NullKeyword),
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

/// Represents an array that may contain elements (ex. `[]`, `[5, 6]`).
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

impl Comment {
    /// Gets the text of the comment.
    pub fn text(&self) -> &str {
        match self {
            Comment::Line(line) => line.text.as_ref(),
            Comment::Block(line) => line.text.as_ref(),
        }
    }
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

macro_rules! impl_ranged {
    ($($node_name:ident),*) => {
        $(
            impl Ranged for $node_name {
                fn range(&self) -> &Range {
                    &self.range
                }
            }
        )*
    };
}

// Implement Traits

impl_ranged![StringLit, NumberLit, BooleanLit, NullKeyword, Object, ObjectProp, Array, CommentLine, CommentBlock];

impl Ranged for Value {
    fn range(&self) -> &Range {
        match self {
            Value::Array(node) => node.range(),
            Value::BooleanLit(node) => node.range(),
            Value::NullKeyword(node) => node.range(),
            Value::NumberLit(node) => node.range(),
            Value::Object(node) => node.range(),
            Value::StringLit(node) => node.range(),
        }
    }
}

macro_rules! generate_node {
    ($($node_name:ident),*) => {
        impl<'a> Ranged for Node<'a> {
            fn range(&self) -> &Range {
                match self {
                    $(Node::$node_name(node) => node.range()),*
                }
            }
        }

        $(
        impl<'a> From<&'a $node_name> for Node<'a> {
            fn from(node: &'a $node_name) -> Node<'a> {
                Node::$node_name(node)
            }
        }
        )*
    };
}

generate_node![StringLit, NumberLit, BooleanLit, NullKeyword, Object, ObjectProp, Array];

impl<'a> From<&'a Value> for Node<'a> {
    fn from(value: &'a Value) -> Node<'a> {
        match value {
            Value::Array(node) => Node::Array(node),
            Value::BooleanLit(node) => Node::BooleanLit(node),
            Value::NullKeyword(node) => Node::NullKeyword(node),
            Value::NumberLit(node) => Node::NumberLit(node),
            Value::Object(node) => Node::Object(node),
            Value::StringLit(node) => Node::StringLit(node),
        }
    }
}
