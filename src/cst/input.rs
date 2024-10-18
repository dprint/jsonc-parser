// todo: write a json! macro for creating one of these easily... it's much more difficult than it seems

#[derive(Debug, Clone)]
pub enum RawCstValue {
  Null,
  Bool(bool),
  Number(String),
  String(String),
  Array(Vec<RawCstValue>),
  Object(Vec<(String, RawCstValue)>),
}

impl RawCstValue {
  pub(crate) fn force_multiline(&self) -> bool {
    match self {
      RawCstValue::Null | RawCstValue::Bool(_) | RawCstValue::Number(_) | RawCstValue::String(_) => false,
      RawCstValue::Array(v) => v.iter().any(|v| v.is_object_or_array_with_elements()),
      RawCstValue::Object(v) => !v.is_empty(),
    }
  }

  fn is_object_or_array_with_elements(&self) -> bool {
    match self {
      RawCstValue::Null | RawCstValue::Bool(_) | RawCstValue::Number(_) | RawCstValue::String(_) => false,
      RawCstValue::Array(v) => !v.is_empty(),
      RawCstValue::Object(v) => !v.is_empty(),
    }
  }
}
