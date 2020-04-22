use std::collections::HashMap;
use std::rc::Rc;
use super::scanner::Scanner;
use super::common::{ImmutableString, Range};
use super::tokens::{Token, TokenAndRange};
use super::ast::*;
use super::errors::*;

/// Result of parsing the text.
pub struct ParseResult {
    /// Collection of comments in the text.
    ///
    /// Remarks: The key is the start and end position of the tokens.
    pub comments: HashMap<usize, Rc<Vec<Comment>>>,
    /// The JSON value the text contained.
    pub value: Option<Value>,
    /// Collection of tokens (excluding any comments).
    pub tokens: Vec<TokenAndRange>,
}

struct Context {
    scanner: Scanner,
    comments: HashMap<usize, Rc<Vec<Comment>>>,
    current_comments: Option<Vec<Comment>>,
    last_token_end: usize,
    range_stack: Vec<Range>,
    tokens: Vec<TokenAndRange>,
}

impl Context {
    pub fn scan(&mut self) -> Result<Option<Token>, ParseError> {
        let previous_end = self.last_token_end;
        let token = self.scan_handling_comments()?;
        self.last_token_end = self.scanner.token_end();

        // store the comment for the previous token end, and current token start
        if let Some(comments) = self.current_comments.take() {
            let comments = Rc::new(comments);
            self.comments.insert(previous_end, comments.clone());
            self.comments.insert(self.scanner.token_start(), comments);
        }

        // capture the token
        if let Some(token) = &token {
            self.tokens.push(TokenAndRange {
                token: token.clone(),
                range: self.create_range_from_last_token(),
            });
        }

        Ok(token)
    }

    pub fn token(&self) -> Option<Token> {
        self.scanner.token()
    }

    pub fn start_range(&mut self) {
        self.range_stack.push(Range {
            start: self.scanner.token_start(),
            start_line: self.scanner.token_start_line(),
            end: 0,
            end_line: 0,
        });
    }

    pub fn end_range(&mut self) -> Range {
        let mut range = self.range_stack.pop().expect("Range was popped from the stack, but the stack was empty.");
        range.end = self.scanner.token_end();
        range.end_line = self.scanner.token_end_line();
        range
    }

    pub fn create_range_from_last_token(&self) -> Range {
        Range {
            start: self.scanner.token_start(),
            end: self.scanner.token_end(),
            start_line: self.scanner.token_start_line(),
            end_line: self.scanner.token_end_line(),
        }
    }

    pub fn create_parse_error(&self, text: &str) -> ParseError {
        ParseError::new(self.scanner.token_start(), text)
    }

    fn scan_handling_comments(&mut self) -> Result<Option<Token>, ParseError> {
        loop {
            let token = self.scanner.scan()?;
            match token {
                Some(Token::CommentLine(text)) => {
                    self.handle_comment(Comment::Line(CommentLine {
                        range: self.create_range_from_last_token(),
                        text,
                    }));
                }
                Some(Token::CommentBlock(text)) => {
                    self.handle_comment(Comment::Block(CommentBlock {
                        range: self.create_range_from_last_token(),
                        text,
                    }));
                }
                _ => return Ok(token),
            }
        }
    }

    fn handle_comment(&mut self, comment: Comment) {
        if let Some(comments) = self.current_comments.as_mut() {
            comments.push(comment);
        } else {
            self.current_comments = Some(vec![comment]);
        }
    }
}

/// Parses a string containing JSONC to an AST with comments and tokens.
///
/// # Example
///
/// ```
/// use jsonc_parser::parse_text;
///
/// let parse_result = parse_text(r#"{ "test": 5 } // test"#);
/// // ...inspect parse_result for value, tokens, and comments here...
/// ```
pub fn parse_text(text: &str) -> Result<ParseResult, ParseError> {
    let mut context = Context {
        scanner: Scanner::new(text),
        comments: HashMap::new(),
        current_comments: None,
        last_token_end: 0,
        range_stack: Vec::new(),
        tokens: Vec::new(),
    };
    context.scan()?;
    let value = parse_value(&mut context)?;

    if context.scan()?.is_some() {
        return Err(context.create_parse_error("Text cannot contain more than one JSON value."));
    }

    debug_assert!(context.range_stack.is_empty());

    Ok(ParseResult {
        comments: context.comments,
        tokens: context.tokens,
        value,
    })
}

fn parse_value(context: &mut Context) -> Result<Option<Value>, ParseError> {
    match context.token() {
        None => Ok(None),
        Some(token) => match token {
            Token::OpenBrace => return Ok(Some(Value::Object(parse_object(context)?))),
            Token::OpenBracket => return Ok(Some(Value::Array(parse_array(context)?))),
            Token::String(value) => return Ok(Some(Value::StringLit(create_string_lit(context, value)))),
            Token::Boolean(value) => return Ok(Some(Value::BooleanLit(create_boolean_lit(context, value)))),
            Token::Number(value) => return Ok(Some(Value::NumberLit(create_number_lit(context, value)))),
            Token::Null => return Ok(Some(Value::NullKeyword(create_null_keyword(context)))),
            Token::CloseBracket => return Err(context.create_parse_error("Unexpected close bracket.")),
            Token::CloseBrace => return Err(context.create_parse_error("Unexpected close brace.")),
            Token::Comma => return Err(context.create_parse_error("Unexpected comma.")),
            Token::Colon => return Err(context.create_parse_error("Unexpected colon.")),
            Token::CommentLine(_) => unreachable!(),
            Token::CommentBlock(_) => unreachable!(),
        }
    }
}

fn parse_object(context: &mut Context) -> Result<Object, ParseError> {
    debug_assert!(context.token() == Some(Token::OpenBrace));
    let mut properties = Vec::new();

    context.start_range();
    context.scan()?;

    loop {
        match context.token() {
            Some(Token::CloseBrace) => break,
            Some(Token::String(prop_name)) => {
                properties.push(parse_object_property(context, prop_name)?);
            }
            None => return Err(context.create_parse_error("Unterminated array literal.")),
            _ => return Err(context.create_parse_error("Unexpected token in array literal.")),
        }

        // skip the comma
        match context.scan()? {
            Some(Token::Comma) => { context.scan()?; },
            _ => {}
        }
    }

    Ok(Object {
        range: context.end_range(),
        properties,
    })
}

fn parse_object_property(context: &mut Context, prop_name: ImmutableString) -> Result<ObjectProp, ParseError> {
    context.start_range();

    let name = create_string_lit(context, prop_name);

    match context.scan()? {
        Some(Token::Colon) => {},
        _ => return Err(context.create_parse_error("Expected a colon after the string in an object property.")),
    }

    context.scan()?;
    let value = parse_value(context)?;

    match value {
        Some(value) => Ok(ObjectProp {
            range: context.end_range(),
            name,
            value,
        }),
        None => Err(context.create_parse_error("Expected value after colon in object property.")),
    }
}

fn parse_array(context: &mut Context) -> Result<Array, ParseError> {
    debug_assert!(context.token() == Some(Token::OpenBracket));
    let mut elements = Vec::new();

    context.start_range();
    context.scan()?;

    loop {
        match context.token() {
            Some(Token::CloseBracket) => break,
            None => return Err(context.create_parse_error("Unterminated array literal.")),
            _ => match parse_value(context)? {
                Some(value) => elements.push(value),
                None => return Err(context.create_parse_error("Unterminated array literal.")),
            }
        }

        // skip the comma
        match context.scan()? {
            Some(Token::Comma) => { context.scan()?; },
            _ => {}
        }
    }

    Ok(Array {
        range: context.end_range(),
        elements,
    })
}

// factory functions

fn create_string_lit(context: &Context, value: ImmutableString) -> StringLit {
    StringLit {
        range: context.create_range_from_last_token(),
        value,
    }
}

fn create_boolean_lit(context: &Context, value: bool) -> BooleanLit {
    BooleanLit {
        range: context.create_range_from_last_token(),
        value,
    }
}

fn create_number_lit(context: &Context, value: ImmutableString) -> NumberLit {
    NumberLit {
        range: context.create_range_from_last_token(),
        value,
    }
}

fn create_null_keyword(context: &Context) -> NullKeyword {
    NullKeyword {
        range: context.create_range_from_last_token(),
    }
}
