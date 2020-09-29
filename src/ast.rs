use super::common::{ImmutableString, Range, Ranged};

/// JSON value.
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    StringLit(StringLit),
    NumberLit(NumberLit),
    BooleanLit(BooleanLit),
    Object(Object),
    Array(Array),
    NullKeyword(NullKeyword),
}

/// Node that can appear in the AST.
#[derive(Debug, PartialEq, Clone)]
pub enum Node<'a> {
    StringLit(&'a StringLit),
    NumberLit(&'a NumberLit),
    BooleanLit(&'a BooleanLit),
    Object(&'a Object),
    ObjectProp(&'a ObjectProp),
    Array(&'a Array),
    NullKeyword(&'a NullKeyword),
    WordLit(&'a WordLit),
}

impl<'a> Node<'a> {
    /// Gets the node kind.
    pub fn kind(&self) -> NodeKind {
        match self {
            Node::StringLit(_) => NodeKind::StringLit,
            Node::NumberLit(_) => NodeKind::NumberLit,
            Node::BooleanLit(_) => NodeKind::BooleanLit,
            Node::Object(_) => NodeKind::Object,
            Node::ObjectProp(_) => NodeKind::ObjectProp,
            Node::Array(_) => NodeKind::Array,
            Node::NullKeyword(_) => NodeKind::NullKeyword,
            Node::WordLit(_) => NodeKind::WordLit,
        }
    }
}

/// Kind of AST node.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NodeKind {
    StringLit,
    NumberLit,
    BooleanLit,
    Object,
    ObjectProp,
    Array,
    NullKeyword,
    WordLit,
}

/// Node surrounded in double quotes (ex. `"my string"`).
#[derive(Debug, PartialEq, Clone)]
pub struct StringLit {
    pub range: Range,
    pub value: ImmutableString,
}

/// A string that's not in quotes.
/// Usually the appearance of this would be a parsing error.
#[derive(Debug, PartialEq, Clone)]
pub struct WordLit {
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

macro_rules! generate_take {
    ($self:ident, $name:ident, $value_type:ident) => {
        // there must be some better code that could be written here...
        if let Some(pos) = $self.properties.iter().position(|p| p.name.as_str() == $name) {
            if let Value::$value_type(_) = &$self.properties[pos].value {
                if let Value::$value_type(node) = $self.properties.remove(pos).value {
                    Some(node)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    };
}

macro_rules! generate_get {
    ($self:ident, $name:ident, $value_type:ident) => {
        $self
            .properties
            .iter()
            .filter(|p| p.name.as_str() == $name)
            .map(|p| {
                if let Value::$value_type(node) = &p.value {
                    Some(node)
                } else {
                    None
                }
            })
            .next()
            .flatten()
    };
}

impl Object {
    /// Gets a property value in the object by its name.
    pub fn get(&self, name: &str) -> Option<&ObjectProp> {
        self.properties.iter().filter(|p| p.name.as_str() == name).next()
    }

    /// Gets a string property value from the object by name.
    /// Returns `None` when not a string or it doesn't exist.
    pub fn get_string(&self, name: &str) -> Option<&StringLit> {
        generate_get!(self, name, StringLit)
    }

    /// Gets a number property value from the object by name.
    /// Returns `None` when not a number or it doesn't exist.
    pub fn get_number(&self, name: &str) -> Option<&NumberLit> {
        generate_get!(self, name, NumberLit)
    }

    /// Gets a boolean property value from the object by name.
    /// Returns `None` when not a boolean or it doesn't exist.
    pub fn get_boolean(&self, name: &str) -> Option<&BooleanLit> {
        generate_get!(self, name, BooleanLit)
    }

    /// Gets an object property value from the object by name.
    /// Returns `None` when not an object or it doesn't exist.
    pub fn get_object(&self, name: &str) -> Option<&Object> {
        generate_get!(self, name, Object)
    }

    /// Gets an array property value from the object by name.
    /// Returns `None` when not an array or it doesn't exist.
    pub fn get_array(&self, name: &str) -> Option<&Array> {
        generate_get!(self, name, Array)
    }

    /// Takes a value from the object by name.
    /// Returns `None` when it doesn't exist.
    pub fn take(&mut self, name: &str) -> Option<ObjectProp> {
        if let Some(pos) = self.properties.iter().position(|p| p.name.as_str() == name) {
            Some(self.properties.remove(pos))
        } else {
            None
        }
    }

    /// Takes a string property value from the object by name.
    /// Returns `None` when not a string or it doesn't exist.
    pub fn take_string(&mut self, name: &str) -> Option<StringLit> {
        generate_take!(self, name, StringLit)
    }

    /// Takes a number property value from the object by name.
    /// Returns `None` when not a number or it doesn't exist.
    pub fn take_number(&mut self, name: &str) -> Option<NumberLit> {
        generate_take!(self, name, NumberLit)
    }

    /// Takes a boolean property value from the object by name.
    /// Returns `None` when not a boolean or it doesn't exist.
    pub fn take_boolean(&mut self, name: &str) -> Option<BooleanLit> {
        generate_take!(self, name, BooleanLit)
    }

    /// Takes an object property value from the object by name.
    /// Returns `None` when not an object or it doesn't exist.
    pub fn take_object(&mut self, name: &str) -> Option<Object> {
        generate_take!(self, name, Object)
    }

    /// Takes an array property value from the object by name.
    /// Returns `None` when not an array or it doesn't exist.
    pub fn take_array(&mut self, name: &str) -> Option<Array> {
        generate_take!(self, name, Array)
    }
}

/// Represents an object property (ex. `"prop": []`).
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectProp {
    pub range: Range,
    pub name: ObjectPropName,
    pub value: Value,
}

/// Represents an object property name that may or may not be in quotes.
#[derive(Debug, PartialEq, Clone)]
pub enum ObjectPropName {
    String(StringLit),
    Word(WordLit),
}

impl ObjectPropName {
    /// Converts the object property name into a string.
    pub fn into_string(self) -> String {
        match self {
            ObjectPropName::String(lit) => lit.value.into_string(),
            ObjectPropName::Word(lit) => lit.value.into_string(),
        }
    }

    /// Gets the object property name as a string.
    pub fn as_str(&self) -> &str {
        match self {
            ObjectPropName::String(lit) => lit.value.as_str(),
            ObjectPropName::Word(lit) => lit.value.as_str(),
        }
    }
}

/// Represents an array that may contain elements (ex. `[]`, `[5, 6]`).
#[derive(Debug, PartialEq, Clone)]
pub struct Array {
    pub range: Range,
    pub elements: Vec<Value>,
}

/// Kind of JSONC comment.
#[derive(Debug, PartialEq, Clone)]
pub enum CommentKind {
    Line,
    Block,
}

/// JSONC comment.
#[derive(Debug, PartialEq, Clone)]
pub enum Comment {
    Line(CommentLine),
    Block(CommentBlock),
}

impl Comment {
    /// Gets the text of the comment.
    pub fn text(&self) -> &str {
        match self {
            Comment::Line(line) => line.text.as_str(),
            Comment::Block(line) => line.text.as_str(),
        }
    }

    /// Gets the comment kind.
    pub fn kind(&self) -> CommentKind {
        match self {
            Comment::Line(_) => CommentKind::Line,
            Comment::Block(_) => CommentKind::Block,
        }
    }
}

impl Ranged for Comment {
    fn range(&self) -> &Range {
        match self {
            Comment::Line(line) => line.range(),
            Comment::Block(line) => line.range(),
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

// Object Property Name

impl<'a> From<&'a ObjectPropName> for Node<'a> {
    fn from(object_prop_name: &'a ObjectPropName) -> Node<'a> {
        match object_prop_name {
            ObjectPropName::String(lit) => lit.into(),
            ObjectPropName::Word(lit) => lit.into(),
        }
    }
}

impl Ranged for ObjectPropName {
    fn range(&self) -> &Range {
        match self {
            ObjectPropName::String(lit) => lit.range(),
            ObjectPropName::Word(lit) => lit.range(),
        }
    }
}

// Implement Traits

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

impl_ranged![
    StringLit,
    WordLit,
    NumberLit,
    BooleanLit,
    NullKeyword,
    Object,
    ObjectProp,
    Array,
    CommentLine,
    CommentBlock
];

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

generate_node![
    StringLit,
    WordLit,
    NumberLit,
    BooleanLit,
    NullKeyword,
    Object,
    ObjectProp,
    Array
];

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

#[cfg(test)]
mod test {
    use super::*;
    use crate::{parse_to_ast, ParseOptions};

    #[test]
    fn it_should_take() {
        let ast = parse_to_ast(
            "{'prop': 'asdf', 'other': 'text'}",
            &ParseOptions {
                tokens: false,
                comments: false,
            },
        )
        .unwrap();
        let mut obj = match ast.value {
            Some(Value::Object(obj)) => obj,
            _ => unreachable!(),
        };

        assert_eq!(obj.properties.len(), 2);
        assert_eq!(obj.take_string("asdf"), None);
        assert_eq!(obj.properties.len(), 2);
        assert_eq!(obj.take_number("prop"), None);
        assert_eq!(obj.properties.len(), 2);
        assert_eq!(obj.take_string("prop").is_some(), true);
        assert_eq!(obj.properties.len(), 1);
        assert_eq!(obj.take("something"), None);
        assert_eq!(obj.properties.len(), 1);
        assert_eq!(obj.take("other").is_some(), true);
        assert_eq!(obj.properties.len(), 0);
    }

    #[test]
    fn it_should_get() {
        let ast = parse_to_ast(
            "{'prop': 'asdf'}",
            &ParseOptions {
                tokens: false,
                comments: false,
            },
        )
        .unwrap();
        let obj = match ast.value {
            Some(Value::Object(obj)) => obj,
            _ => unreachable!(),
        };

        assert_eq!(obj.properties.len(), 1);
        assert_eq!(obj.get_string("asdf"), None);
        assert_eq!(obj.get_string("prop").is_some(), true);
        assert_eq!(obj.get("asdf"), None);
        assert_eq!(obj.properties.len(), 1);
    }
}
