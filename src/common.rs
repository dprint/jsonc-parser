/// Positional information about a start and end point in the text.
#[derive(Debug, PartialEq, Clone)]
pub struct Range {
    /// Start position of the node in the text.
    pub start: usize,
    /// End position of the node in the text.
    pub end: usize,
    /// Line of the start position of the node in the text.
    pub start_line: usize,
    /// Line of the end position of the node in the text.
    pub end_line: usize,
}

impl Range {
    /// Gets the end position minus the start of the range.
    pub fn width(&self) -> usize {
        self.end - self.start
    }
}

/// Represents an object that has a range in the text.
pub trait Ranged {
    /// Gets the range.
    fn range(&self) -> &Range;

    /// Gets the index of the first character in the text.
    fn start(&self) -> usize {
        self.range().start
    }
    /// Gets the line number of the start position in the text.
    fn start_line(&self) -> usize {
        self.range().start_line
    }
    /// Gets the index after the last character in the text.
    fn end(&self) -> usize {
        self.range().end
    }
    /// Gets the line number of the end position in the text.
    fn end_line(&self) -> usize {
        self.range().end_line
    }
    /// Gets the text from the provided string.
    fn text<'a>(&self, text: &'a str) -> &'a str {
        &text[self.start()..self.end()]
    }
    /// Gets the start position.
    fn start_position(&self) -> Position {
        let range = self.range();
        Position::new(range.start, range.start_line)
    }
    /// Gets the end position.
    fn end_position(&self) -> Position {
        let range = self.range();
        Position::new(range.end, range.end_line)
    }
    /// Gets the end position minus the start of the range.
    fn width(&self) -> usize {
        self.range().width()
    }
}

/// Ranged value that specifies a specific position in the file.
pub struct Position {
    pub range: Range,
}

impl Position {
    /// Creates a new position at the specified position and line.
    pub fn new(pos: usize, line: usize) -> Position {
        Position {
            range: Range {
                start: pos,
                end: pos,
                start_line: line,
                end_line: line,
            },
        }
    }
}

impl Ranged for Position {
    fn range(&self) -> &Range {
        &self.range
    }
}
