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

#[macro_export]
macro_rules! json {
  (null) => {
    $crate::cst::RawCstValue::Null
  };

  (true) => {
    $crate::cst::RawCstValue::Bool(true)
  };

  (false) => {
    $crate::cst::RawCstValue::Bool(false)
  };

  ($num:literal) => {
    $crate::cst::RawCstValue::Number($num.to_string())
  };

  ($str:literal) => {
    $crate::cst::RawCstValue::String($str.to_string())
  };

  ([ $($elems:tt),* $(,)? ]) => {
    $crate::cst::RawCstValue::Array(vec![
      $(json!($elems)),*
    ])
  };

  ({ $($key:tt : $value:tt),* $(,)? }) => {
    $crate::cst::RawCstValue::Object(vec![
      $(($key.to_string(), json!($value))),*
    ])
  };
}
