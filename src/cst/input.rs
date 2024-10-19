#[derive(Debug, Clone)]
pub enum CstInputValue {
  Null,
  Bool(bool),
  Number(String),
  String(String),
  Array(Vec<CstInputValue>),
  Object(Vec<(String, CstInputValue)>),
}

impl CstInputValue {
  pub(crate) fn force_multiline(&self) -> bool {
    match self {
      CstInputValue::Null | CstInputValue::Bool(_) | CstInputValue::Number(_) | CstInputValue::String(_) => false,
      CstInputValue::Array(v) => v.iter().any(|v| v.is_object_or_array_with_elements()),
      CstInputValue::Object(v) => !v.is_empty(),
    }
  }

  fn is_object_or_array_with_elements(&self) -> bool {
    match self {
      CstInputValue::Null | CstInputValue::Bool(_) | CstInputValue::Number(_) | CstInputValue::String(_) => false,
      CstInputValue::Array(v) => !v.is_empty(),
      CstInputValue::Object(v) => !v.is_empty(),
    }
  }
}

#[macro_export]
macro_rules! value {
  (null) => {
    $crate::cst::CstInputValue::Null
  };

  (true) => {
    $crate::cst::CstInputValue::Bool(true)
  };

  (false) => {
    $crate::cst::CstInputValue::Bool(false)
  };

  ($num:literal) => {
    $crate::cst::CstInputValue::Number($num.to_string())
  };

  ($str:literal) => {
    $crate::cst::CstInputValue::String($str.to_string())
  };

  ([ $($elems:tt),* $(,)? ]) => {
    $crate::cst::CstInputValue::Array(vec![
      $(value!($elems)),*
    ])
  };

  ({ $($key:tt : $value:tt),* $(,)? }) => {
    $crate::cst::CstInputValue::Object(vec![
      $(($key.to_string(), value!($value))),*
    ])
  };
}
