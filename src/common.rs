/// Positional information about a start and end point in the text.
#[derive(Debug, PartialEq, Clone)]
pub struct Range {
  /// Start position of the node in the text.
  pub start: Position,
  /// End position of the node in the text.
  pub end: Position,
}

impl Range {
  /// Gets the end byte index minus the start byte index of the range.
  pub fn width(&self) -> usize {
    self.end.index - self.start.index
  }
}

/// Represents an object that has a range in the text.
pub trait Ranged {
  /// Gets the range.
  fn range(&self) -> &Range;

  /// Gets the byte index of the first character in the text.
  fn start(&self) -> usize {
    self.range().start.index
  }

  /// Gets the line number of the start position in the text.
  fn start_line(&self) -> usize {
    self.range().start.line
  }

  /// Gets the byte index after the last character in the text.
  fn end(&self) -> usize {
    self.range().end.index
  }

  /// Gets the line number of the end position in the text.
  fn end_line(&self) -> usize {
    self.range().end.line
  }

  /// Gets the text from the provided string.
  fn text<'a>(&self, text: &'a str) -> &'a str {
    &text[self.start()..self.end()]
  }

  /// Gets the start position.
  fn start_position(&self) -> &Position {
    let range = self.range();
    &range.start
  }

  /// Gets the end byte position.
  fn end_position(&self) -> &Position {
    let range = self.range();
    &range.end
  }

  /// Gets the end byte index minus the start byte index of the range.
  fn width(&self) -> usize {
    self.range().width()
  }
}

/// Ranged value that specifies a specific position in the file.
#[derive(Debug, PartialEq, Clone)]
pub struct Position {
  /// Byte index of the node in the text.
  pub index: usize,
  /// Line of the position of the node in the text.
  pub line: usize,
}

impl Position {
  /// Creates a new position at the specified position and line.
  pub fn new(pos: usize, line: usize) -> Position {
    Position { index: pos, line }
  }

  /// Gets the column index from the provided file text.
  pub fn column_index(&self, file_text: &str) -> usize {
    let mut column_number = 0;
    for (indice, c) in file_text.char_indices() {
      if c == '\n' {
        column_number = 0;
      } else if indice >= self.index {
        break;
      } else {
        column_number += 1;
      }
    }
    column_number
  }
}
