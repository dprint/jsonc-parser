/// Error that could occur while parsing or tokenizing.
#[derive(Debug, PartialEq)]
pub struct ParseError {
    pub pos: usize,
    pub message: String,
}

impl ParseError {
    pub(super) fn new(pos: usize, message: &str) -> ParseError {
        ParseError {
            pos,
            message: String::from(message),
        }
    }
}
