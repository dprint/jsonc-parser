use ::serde::de::DeserializeSeed;
use ::serde::de::EnumAccess;
use ::serde::de::IntoDeserializer;
use ::serde::de::MapAccess;
use ::serde::de::SeqAccess;
use ::serde::de::VariantAccess;
use ::serde::de::Visitor;
use ::serde::forward_to_deserialize_any;

use super::CollectOptions;
use super::ParseOptions;
use super::ast::Array;
use super::ast::NumberLit;
use super::ast::ObjectProp;
use super::ast::Value;
use super::common::Ranged;
use super::errors::ParseError;
use super::errors::ParseErrorKind;
use super::parse_to_ast;

/// Parses a string containing JSONC to a `serde_json::Value` or any
/// type that implements `serde::Deserialize`.
///
/// Requires the "serde" cargo feature:
///
/// ```toml
/// jsonc-parser = { version = "...", features = ["serde"] }
/// ```
///
/// # Example
///
/// Parsing to a `serde_json::Value`:
///
/// ```rs
/// use jsonc_parser::parse_to_serde_value;
///
/// let json_value = parse_to_serde_value::<serde_json::Value>(
///   r#"{ "test": 5 } // test"#,
///   &Default::default(),
/// ).unwrap();
/// ```
///
/// Or to a concrete type:
///
/// ```rs
/// use jsonc_parser::parse_to_serde_value;
///
/// #[derive(serde::Deserialize)]
/// struct Config {
///   test: u32,
/// }
///
/// let config = parse_to_serde_value::<Config>(
///   r#"{ "test": 5 } // test"#,
///   &Default::default(),
/// ).unwrap();
/// ```
pub fn parse_to_serde_value<T: ::serde::de::DeserializeOwned>(
  text: &str,
  parse_options: &ParseOptions,
) -> Result<Option<T>, ParseError> {
  let value = parse_to_ast(
    text,
    &CollectOptions {
      comments: crate::CommentCollectionStrategy::Off,
      tokens: false,
    },
    parse_options,
  )?
  .value;
  match value {
    Some(v) => {
      let deserializer = AstDeserializer { value: v, text };
      T::deserialize(deserializer).map(Some)
    }
    None => Ok(None),
  }
}

impl ::serde::de::Error for ParseError {
  fn custom<T: std::fmt::Display>(msg: T) -> Self {
    ParseError::custom_err(msg.to_string())
  }
}

struct AstDeserializer<'a> {
  value: Value<'a>,
  text: &'a str,
}

impl<'de> ::serde::Deserializer<'de> for AstDeserializer<'de> {
  type Error = ParseError;

  fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
    let range = self.value.range();
    let text = self.text;
    let result = match self.value {
      Value::NullKeyword(_) => visitor.visit_unit(),
      Value::BooleanLit(b) => visitor.visit_bool(b.value),
      Value::NumberLit(n) => visit_number(n, visitor),
      Value::StringLit(s) => match s.value {
        std::borrow::Cow::Borrowed(b) => visitor.visit_borrowed_str(b),
        std::borrow::Cow::Owned(o) => visitor.visit_string(o),
      },
      Value::Array(arr) => visit_array(arr, text, visitor),
      Value::Object(obj) => visit_object(obj.properties, text, visitor),
    };
    result.map_err(|e| e.with_position(range, text))
  }

  fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
    if matches!(&self.value, Value::NullKeyword(_)) {
      visitor.visit_none()
    } else {
      visitor.visit_some(self)
    }
  }

  fn deserialize_enum<V: Visitor<'de>>(
    self,
    _name: &'static str,
    _variants: &'static [&'static str],
    visitor: V,
  ) -> Result<V::Value, Self::Error> {
    let range = self.value.range();
    let text = self.text;
    let result = match self.value {
      Value::StringLit(s) => {
        let variant: String = s.value.into_owned();
        visitor.visit_enum(variant.into_deserializer())
      }
      Value::Object(obj) => {
        if obj.properties.len() != 1 {
          return Err(ParseError::new(
            range,
            ParseErrorKind::Custom("expected an object with exactly one property for enum".to_string()),
            text,
          ));
        }
        let prop = obj.properties.into_iter().next().unwrap();
        visitor.visit_enum(ObjectEnumDeserializer {
          variant: prop.name.into_string(),
          value: prop.value,
          text,
        })
      }
      _ => {
        return Err(ParseError::new(
          range,
          ParseErrorKind::Custom("expected a string or object for enum".to_string()),
          text,
        ));
      }
    };
    result.map_err(|e| e.with_position(range, text))
  }

  fn deserialize_newtype_struct<V: Visitor<'de>>(
    self,
    _name: &'static str,
    visitor: V,
  ) -> Result<V::Value, Self::Error> {
    visitor.visit_newtype_struct(self)
  }

  forward_to_deserialize_any! {
    bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64
    char str string bytes byte_buf unit unit_struct
    seq tuple tuple_struct map struct identifier ignored_any
  }
}

// number handling

fn visit_number<'de, V: Visitor<'de>>(num: NumberLit<'_>, visitor: V) -> Result<V::Value, ParseError> {
  let raw = num.value;

  // handle hexadecimal
  let trimmed = raw.trim_start_matches(['-', '+']);
  if trimmed.len() > 2 && (trimmed.starts_with("0x") || trimmed.starts_with("0X")) {
    let hex_part = &trimmed[2..];
    match i64::from_str_radix(hex_part, 16) {
      Ok(val) => {
        let val = if raw.starts_with('-') { -val } else { val };
        return visitor.visit_i64(val);
      }
      Err(_) => return visitor.visit_str(raw),
    }
  }

  // strip unary plus
  let num_str = raw.trim_start_matches('+');

  if let Ok(v) = num_str.parse::<i64>() {
    return visitor.visit_i64(v);
  }
  if let Ok(v) = num_str.parse::<u64>() {
    return visitor.visit_u64(v);
  }
  if let Ok(v) = num_str.parse::<f64>() {
    return visitor.visit_f64(v);
  }

  // fallback for unparseable numbers
  visitor.visit_str(raw)
}

// array handling

fn visit_array<'de, V: Visitor<'de>>(arr: Array<'de>, text: &'de str, visitor: V) -> Result<V::Value, ParseError> {
  visitor.visit_seq(ArraySeqAccess {
    iter: arr.elements.into_iter(),
    text,
  })
}

struct ArraySeqAccess<'a> {
  iter: std::vec::IntoIter<Value<'a>>,
  text: &'a str,
}

impl<'de> SeqAccess<'de> for ArraySeqAccess<'de> {
  type Error = ParseError;

  fn next_element_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error> {
    match self.iter.next() {
      Some(value) => seed.deserialize(AstDeserializer { value, text: self.text }).map(Some),
      None => Ok(None),
    }
  }
}

// object handling

fn visit_object<'de, V: Visitor<'de>>(
  properties: Vec<ObjectProp<'de>>,
  text: &'de str,
  visitor: V,
) -> Result<V::Value, ParseError> {
  visitor.visit_map(ObjectMapAccess {
    iter: properties.into_iter(),
    pending_value: None,
    text,
  })
}

struct ObjectMapAccess<'a> {
  iter: std::vec::IntoIter<ObjectProp<'a>>,
  pending_value: Option<Value<'a>>,
  text: &'a str,
}

impl<'de> MapAccess<'de> for ObjectMapAccess<'de> {
  type Error = ParseError;

  fn next_key_seed<K: DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error> {
    match self.iter.next() {
      Some(prop) => {
        let key = prop.name.into_string();
        self.pending_value = Some(prop.value);
        seed
          .deserialize(<String as IntoDeserializer<Self::Error>>::into_deserializer(key))
          .map(Some)
      }
      None => Ok(None),
    }
  }

  fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, Self::Error> {
    let value = self
      .pending_value
      .take()
      .expect("next_value_seed called before next_key_seed");
    seed.deserialize(AstDeserializer { value, text: self.text })
  }
}

// enum handling

struct ObjectEnumDeserializer<'a> {
  variant: String,
  value: Value<'a>,
  text: &'a str,
}

impl<'de> EnumAccess<'de> for ObjectEnumDeserializer<'de> {
  type Error = ParseError;
  type Variant = ObjectVariantDeserializer<'de>;

  fn variant_seed<V: DeserializeSeed<'de>>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error> {
    let variant = seed.deserialize(<String as IntoDeserializer<Self::Error>>::into_deserializer(
      self.variant,
    ))?;
    Ok((
      variant,
      ObjectVariantDeserializer {
        value: self.value,
        text: self.text,
      },
    ))
  }
}

struct ObjectVariantDeserializer<'a> {
  value: Value<'a>,
  text: &'a str,
}

impl<'de> VariantAccess<'de> for ObjectVariantDeserializer<'de> {
  type Error = ParseError;

  fn unit_variant(self) -> Result<(), Self::Error> {
    let range = self.value.range();
    ::serde::Deserialize::deserialize(AstDeserializer {
      value: self.value,
      text: self.text,
    })
    .map_err(|e: ParseError| e.with_position(range, self.text))
  }

  fn newtype_variant_seed<T: DeserializeSeed<'de>>(self, seed: T) -> Result<T::Value, Self::Error> {
    seed.deserialize(AstDeserializer {
      value: self.value,
      text: self.text,
    })
  }

  fn tuple_variant<V: Visitor<'de>>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error> {
    match self.value {
      Value::Array(arr) => visit_array(arr, self.text, visitor),
      other => {
        let range = other.range();
        Err(ParseError::new(
          range,
          ParseErrorKind::Custom("expected an array for tuple variant".to_string()),
          self.text,
        ))
      }
    }
  }

  fn struct_variant<V: Visitor<'de>>(
    self,
    _fields: &'static [&'static str],
    visitor: V,
  ) -> Result<V::Value, Self::Error> {
    match self.value {
      Value::Object(obj) => visit_object(obj.properties, self.text, visitor),
      other => {
        let range = other.range();
        Err(ParseError::new(
          range,
          ParseErrorKind::Custom("expected an object for struct variant".to_string()),
          self.text,
        ))
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use serde_json::Value as SerdeValue;
  use std::str::FromStr;

  use super::*;

  #[test]
  fn it_should_error_when_has_error() {
    assert_has_error(
      "[][]",
      "Text cannot contain more than one JSON value on line 1 column 3",
    );
  }

  fn assert_has_error(text: &str, message: &str) {
    let result = parse_to_serde_value::<SerdeValue>(text, &Default::default());
    match result {
      Ok(_) => panic!("Expected error, but did not find one."),
      Err(err) => assert_eq!(err.to_string(), message),
    }
  }

  #[test]
  fn it_should_parse_to_serde_value() {
    let result = parse_to_serde_value::<SerdeValue>(
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

  #[test]
  fn it_should_parse_hexadecimal_numbers_to_decimal() {
    let result = parse_to_serde_value::<SerdeValue>(
      r#"{
        "hex1": 0x7DF,
        "hex2": 0xFF,
        "hex3": 0x10
      }"#,
      &Default::default(),
    )
    .unwrap();

    let mut expected_value = serde_json::map::Map::new();
    expected_value.insert("hex1".to_string(), SerdeValue::Number(serde_json::Number::from(2015)));
    expected_value.insert("hex2".to_string(), SerdeValue::Number(serde_json::Number::from(255)));
    expected_value.insert("hex3".to_string(), SerdeValue::Number(serde_json::Number::from(16)));

    assert_eq!(result, Some(SerdeValue::Object(expected_value)));
  }

  #[test]
  fn it_should_parse_unary_plus_numbers() {
    let result = parse_to_serde_value::<SerdeValue>(
      r#"{
        "pos1": +42,
        "pos2": +0.5,
        "pos3": +1e10
      }"#,
      &Default::default(),
    )
    .unwrap();

    let mut expected_value = serde_json::map::Map::new();
    expected_value.insert("pos1".to_string(), SerdeValue::Number(serde_json::Number::from(42)));
    expected_value.insert(
      "pos2".to_string(),
      SerdeValue::Number(serde_json::Number::from_str("0.5").unwrap()),
    );
    expected_value.insert(
      "pos3".to_string(),
      SerdeValue::Number(serde_json::Number::from_str("1e10").unwrap()),
    );

    assert_eq!(result, Some(SerdeValue::Object(expected_value)));
  }

  #[test]
  fn it_should_deserialize_to_struct() {
    #[derive(::serde::Deserialize, Debug, PartialEq)]
    #[serde(crate = "::serde")]
    struct Config {
      name: String,
      value: u32,
      enabled: bool,
    }

    let result: Option<Config> = parse_to_serde_value(
      r#"{ "name": "test", "value": 42, "enabled": true }"#,
      &Default::default(),
    )
    .unwrap();

    assert_eq!(
      result,
      Some(Config {
        name: "test".to_string(),
        value: 42,
        enabled: true,
      })
    );
  }

  #[test]
  fn it_should_report_position_on_type_error() {
    #[derive(::serde::Deserialize, Debug)]
    #[serde(crate = "::serde")]
    #[allow(dead_code)]
    struct Config {
      name: String,
    }

    let text = r#"{
  "name": true
}"#;
    let err = parse_to_serde_value::<Config>(text, &Default::default()).unwrap_err();
    // the error should point at `true` (line 2, column 11)
    assert_eq!(err.line_display(), 2);
    assert_eq!(err.column_display(), 11);
    assert!(err.to_string().contains("invalid type"), "got: {}", err);
  }

  #[test]
  fn it_should_deserialize_option_fields() {
    #[derive(::serde::Deserialize, Debug, PartialEq)]
    #[serde(crate = "::serde")]
    struct Config {
      a: Option<u32>,
      b: Option<u32>,
    }

    let result: Option<Config> = parse_to_serde_value(r#"{ "a": 5, "b": null }"#, &Default::default()).unwrap();

    assert_eq!(result, Some(Config { a: Some(5), b: None }));
  }

  #[test]
  fn it_should_deserialize_enum() {
    #[derive(::serde::Deserialize, Debug, PartialEq)]
    #[serde(crate = "::serde")]
    enum Color {
      Red,
      Green,
      Blue,
    }

    #[derive(::serde::Deserialize, Debug, PartialEq)]
    #[serde(crate = "::serde")]
    struct Config {
      color: Color,
    }

    let result: Option<Config> = parse_to_serde_value(r#"{ "color": "Red" }"#, &Default::default()).unwrap();

    assert_eq!(result, Some(Config { color: Color::Red }));
  }

  #[test]
  fn it_should_deserialize_complex_enum() {
    #[derive(::serde::Deserialize, Debug, PartialEq)]
    #[serde(crate = "::serde")]
    enum Shape {
      Circle(f64),
      Rectangle { width: f64, height: f64 },
    }

    let result: Option<Shape> = parse_to_serde_value(r#"{ "Circle": 5.0 }"#, &Default::default()).unwrap();
    assert_eq!(result, Some(Shape::Circle(5.0)));

    let result: Option<Shape> = parse_to_serde_value(
      r#"{ "Rectangle": { "width": 3.0, "height": 4.0 } }"#,
      &Default::default(),
    )
    .unwrap();
    assert_eq!(
      result,
      Some(Shape::Rectangle {
        width: 3.0,
        height: 4.0
      })
    );
  }

  #[test]
  fn it_should_return_none_for_empty_input() {
    let result = parse_to_serde_value::<SerdeValue>("", &Default::default()).unwrap();
    assert_eq!(result, None);
  }

  #[test]
  fn it_should_handle_comments_in_jsonc() {
    #[derive(::serde::Deserialize, Debug, PartialEq)]
    #[serde(crate = "::serde")]
    struct Config {
      value: u32,
    }

    let result: Option<Config> = parse_to_serde_value(
      r#"{
        // this is a comment
        "value": 42 /* inline comment */
      }"#,
      &Default::default(),
    )
    .unwrap();

    assert_eq!(result, Some(Config { value: 42 }));
  }
}
