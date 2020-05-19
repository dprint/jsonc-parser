use std::collections::HashMap;
use super::ast;
use super::errors::ParseError;
use super::{parse_to_ast, ParseOptions};

/// A JSON value.
#[derive(Clone, PartialEq, Debug)]
pub enum JsonValue {
    String(String),
    Number(String),
    Boolean(bool),
    Object(JsonObject),
    Array(JsonArray),
    Null,
}

/// A JSON object.
pub type JsonObject = HashMap<String, JsonValue>;
/// A JSON array.
pub type JsonArray = Vec<JsonValue>;

/// Parses a string containing JSONC to a `JsonValue`.
///
/// # Example
///
/// ```
/// use jsonc_parser::parse_to_value;
///
/// let json_value = parse_to_value(r#"{ "test": 5 } // test"#).expect("Should parse.");
/// ```
pub fn parse_to_value(text: &str) -> Result<Option<JsonValue>, ParseError> {
    let value = parse_to_ast(text, &ParseOptions {
        comments: false,
        tokens: false,
    })?.value;
    Ok(value.map(handle_value))
}

fn handle_value(value: ast::Value) -> JsonValue {
    match value {
        ast::Value::StringLit(lit) => JsonValue::String(lit.value.to_string()),
        ast::Value::NumberLit(lit) => JsonValue::Number(lit.value.to_string()),
        ast::Value::BooleanLit(lit) => JsonValue::Boolean(lit.value),
        ast::Value::Object(obj) => JsonValue::Object(handle_object(obj)),
        ast::Value::Array(arr) => JsonValue::Array(handle_array(arr)),
        ast::Value::NullKeyword(_) => JsonValue::Null,
    }
}

fn handle_array(arr: ast::Array) -> JsonArray {
    arr.elements.into_iter().map(|element| {
        handle_value(element)
    }).collect()
}

fn handle_object(obj: ast::Object) -> JsonObject {
    let mut props = HashMap::new();
    for prop in obj.properties.into_iter() {
        let prop_name = prop.name.value.to_string();
        let prop_value = handle_value(prop.value);
        props.insert(prop_name, prop_value);
    }
    props
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_parse_object() {
        let value = parse_to_value(r#"{
    "a": null,
    "b": [null, "text"],
    "c": true,
    "d": 25.55
}"#).unwrap().unwrap();

        let mut object_map = HashMap::new();
        object_map.insert(String::from("a"), JsonValue::Null);
        object_map.insert(String::from("b"), JsonValue::Array(vec![JsonValue::Null, JsonValue::String(String::from("text"))]));
        object_map.insert(String::from("c"), JsonValue::Boolean(true));
        object_map.insert(String::from("d"), JsonValue::Number(String::from("25.55")));
        assert_eq!(value, JsonValue::Object(object_map));
    }

    #[test]
    fn it_should_parse_boolean_false() {
        let value = parse_to_value("false").unwrap().unwrap();
        assert_eq!(value, JsonValue::Boolean(false));
        let value = parse_to_value("true").unwrap().unwrap();
        assert_eq!(value, JsonValue::Boolean(true));
    }

    #[test]
    fn it_should_parse_boolean_true() {
        let value = parse_to_value("true").unwrap().unwrap();
        assert_eq!(value, JsonValue::Boolean(true));
    }

    #[test]
    fn it_should_parse_number() {
        let value = parse_to_value("50").unwrap().unwrap();
        assert_eq!(value, JsonValue::Number(String::from("50")));
    }

    #[test]
    fn it_should_parse_string() {
        let value = parse_to_value(r#""test""#).unwrap().unwrap();
        assert_eq!(value, JsonValue::String(String::from("test")));
    }

    #[test]
    fn it_should_parse_array() {
        let value = parse_to_value(r#"[false, true]"#).unwrap().unwrap();
        assert_eq!(value, JsonValue::Array(vec![JsonValue::Boolean(false), JsonValue::Boolean(true)]));
    }

    #[test]
    fn it_should_parse_null() {
        let value = parse_to_value("null").unwrap().unwrap();
        assert_eq!(value, JsonValue::Null);
    }

    #[test]
    fn it_should_parse_empty() {
        let value = parse_to_value("").unwrap();
        assert_eq!(value.is_none(), true);
    }
}
