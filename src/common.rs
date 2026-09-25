/// Positional information about a start and end point in the text.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Range {
  /// Start position of the node in the text.
  pub start: usize,
  /// End position of the node in the text.
  pub end: usize,
}

impl Range {
  pub fn new(start: usize, end: usize) -> Self {
    Range { start, end }
  }

  pub fn from_byte_index(pos: usize) -> Self {
    Range::new(pos, pos)
  }
}

impl Ranged for Range {
  fn range(&self) -> Range {
    *self
  }
}

/// Adds the digit serde_json needs beside a leading or trailing decimal point (ex. `.5` to `0.5`).
#[cfg(feature = "serde_json")]
pub(crate) fn fill_bare_decimal_point(num: &str) -> std::borrow::Cow<'_, str> {
  let Some(dot) = num.find('.') else {
    return std::borrow::Cow::Borrowed(num);
  };
  let before = num[..dot].ends_with(|c: char| c.is_ascii_digit());
  let after = num[dot + 1..].starts_with(|c: char| c.is_ascii_digit());
  if before && after {
    return std::borrow::Cow::Borrowed(num);
  }
  let mut filled = String::with_capacity(num.len() + 1);
  filled.push_str(&num[..dot]);
  filled.push_str(if before { ".0" } else { "0." });
  filled.push_str(&num[dot + 1..]);
  std::borrow::Cow::Owned(filled)
}

/// Represents an object that has a range in the text.
pub trait Ranged {
  /// Gets the range.
  fn range(&self) -> Range;

  /// Gets the byte index of the first character in the text.
  fn start(&self) -> usize {
    self.range().start
  }

  /// Gets the byte index after the last character in the text.
  fn end(&self) -> usize {
    self.range().end
  }

  /// Gets the text from the provided string.
  fn text<'a>(&self, text: &'a str) -> &'a str {
    let range = self.range();
    &text[range.start..range.end]
  }

  /// Gets the end byte index minus the start byte index of the range.
  fn width(&self) -> usize {
    let range = self.range();
    range.end - range.start
  }
}
