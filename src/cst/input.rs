// todo: write a json! macro for creating one of these easily... it's much more difficult than it seems

#[derive(Debug, Clone)]
pub enum RawCstValue {
  Null,
  Bool(bool),
  Number(String),
  String(String),
  Array(Vec<RawCstValue>),
  Object(Vec<RawCstObjectValue>),
  Comment(String),
}

impl RawCstValue {
  pub(crate) fn force_multiline(&self) -> bool {
    match self {
      RawCstValue::Null | RawCstValue::Bool(_) | RawCstValue::Number(_) | RawCstValue::String(_) => false,
      RawCstValue::Array(_) | RawCstValue::Object(_) | RawCstValue::Comment(_) => true,
    }
  }
}

#[derive(Debug, Clone)]
pub enum RawCstObjectValue {
  Comment(String),
  KeyValue(String, RawCstValue),
}
