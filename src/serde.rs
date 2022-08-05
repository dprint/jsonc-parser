use super::{errors::ParseError, parse_to_ast, CollectOptions, ParseOptions};

/// Parses a string containing JSONC to a `serde_json::Value.
///
/// Requires the "serde" cargo feature:
///
/// ```toml
/// jsonc-parser = { version = "...", features = ["serde"] }
/// ```
///
/// # Example
///
/// ```rs
/// use jsonc_parser::parse_to_serde_value;
///
/// let json_value = parse_to_serde_value(r#"{ "test": 5 } // test"#, &Default::default()).unwrap();
/// ```
pub fn parse_to_serde_value(text: &str, parse_options: &ParseOptions) -> Result<Option<serde_json::Value>, ParseError> {
  let value = parse_to_ast(
    text,
    &CollectOptions {
      comments: false,
      tokens: false,
    },
    parse_options,
  )?
  .value;
  Ok(value.map(|v| v.into()))
}

#[cfg(test)]
mod tests {
  use serde_json::Value as SerdeValue;
  use std::str::FromStr;

  use super::*;

  #[test]
  fn it_should_error_when_has_error() {
    assert_has_error(
      "[][]",
      "Text cannot contain more than one JSON value on line 1 column 3.",
    );
  }

  fn assert_has_error(text: &str, message: &str) {
    let result = parse_to_serde_value(text, &Default::default());
    match result {
      Ok(_) => panic!("Expected error, but did not find one."),
      Err(err) => assert_eq!(err.to_string(), message),
    }
  }

  #[test]
  fn it_should_parse_to_serde_value() {
    let result = parse_to_serde_value(
      r#"{ "a": { "a1": 5 }, "b": [0.3e+025], "c": "c1", "d": true, "e": false, "f": null }"#,
      &Default::default(),
    )
    .unwrap();

    let mut expected_value = serde_json::map::Map::new();
    expected_value.insert("a".to_string(), {
      let mut inner_obj = serde_json::map::Map::new();
      inner_obj.insert(
        "a1".to_string(),
        SerdeValue::Number(serde_json::Number::from_str("5").unwrap()),
      );
      SerdeValue::Object(inner_obj)
    });
    expected_value.insert("b".to_string(), {
      let mut inner_array = Vec::new();
      inner_array.push(SerdeValue::Number(serde_json::Number::from_str("0.3e+025").unwrap()));
      SerdeValue::Array(inner_array)
    });
    expected_value.insert("c".to_string(), SerdeValue::String("c1".to_string()));
    expected_value.insert("d".to_string(), SerdeValue::Bool(true));
    expected_value.insert("e".to_string(), SerdeValue::Bool(false));
    expected_value.insert("f".to_string(), SerdeValue::Null);

    assert_eq!(result, Some(SerdeValue::Object(expected_value)));
  }
}
