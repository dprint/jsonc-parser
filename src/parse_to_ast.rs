use super::ast::*;
use super::common::Range;
use super::errors::*;
use super::scanner::Scanner;
use super::tokens::{Token, TokenAndRange};
use std::borrow::Cow;
use std::collections::HashMap;
use std::rc::Rc;

/// Map where the comments are stored in collections where
/// the key is the previous token end or start of file or
/// next token start or end of the file.
pub type CommentMap<'a> = HashMap<usize, Rc<Vec<Comment<'a>>>>;

/// Options for parsing.
#[derive(Default)]
pub struct ParseOptions {
    /// Whether to include tokens in the result.
    pub tokens: bool,
    /// Whether to include comments in the result.
    pub comments: bool,
}

/// Result of parsing the text.
pub struct ParseResult<'a> {
    /// Collection of comments in the text.
    ///
    /// Provide `comments: true` to the `ParseOptions` for this to have a value.
    ///
    /// Remarks: The key is the start and end position of the tokens.
    pub comments: Option<CommentMap<'a>>,
    /// The JSON value the text contained.
    pub value: Option<Value<'a>>,
    /// Collection of tokens (excluding any comments).
    ///
    /// Provide `tokens: true` to the `ParseOptions` for this to have a value.
    pub tokens: Option<Vec<TokenAndRange<'a>>>,
}

struct Context<'a> {
    scanner: Scanner<'a>,
    comments: Option<CommentMap<'a>>,
    current_comments: Option<Vec<Comment<'a>>>,
    last_token_end: usize,
    range_stack: Vec<Range>,
    tokens: Option<Vec<TokenAndRange<'a>>>,
}

impl<'a> Context<'a> {
    pub fn scan(&mut self) -> Result<Option<Token<'a>>, ParseError> {
        let previous_end = self.last_token_end;
        let token = self.scan_handling_comments()?;
        self.last_token_end = self.scanner.token_end();

        // store the comment for the previous token end, and current token start
        if let Some(comments) = self.comments.as_mut() {
            if let Some(current_comments) = self.current_comments.take() {
                let current_comments = Rc::new(current_comments);
                comments.insert(previous_end, current_comments.clone());
                comments.insert(self.scanner.token_start(), current_comments);
            }
        }

        // capture the token
        if let Some(token) = &token {
            if self.tokens.is_some() {
                let range = self.create_range_from_last_token();
                if let Some(tokens) = self.tokens.as_mut() {
                    tokens.push(TokenAndRange {
                        token: token.clone(),
                        range,
                    });
                }
            }
        }

        Ok(token)
    }

    pub fn token(&self) -> Option<Token<'a>> {
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
        let mut range = self
            .range_stack
            .pop()
            .expect("Range was popped from the stack, but the stack was empty.");
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

    pub fn create_parse_error(&self, message: &str) -> ParseError {
        self.scanner.create_error_for_current_token(message)
    }

    pub fn create_parse_error_for_current_range(&mut self, message: &str) -> ParseError {
        let range = self.end_range();
        self.scanner.create_error_for_range(range, message)
    }

    fn scan_handling_comments(&mut self) -> Result<Option<Token<'a>>, ParseError> {
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

    fn handle_comment(&mut self, comment: Comment<'a>) {
        if self.comments.is_some() {
            if let Some(comments) = self.current_comments.as_mut() {
                comments.push(comment);
            } else {
                self.current_comments = Some(vec![comment]);
            }
        }
    }
}

/// Parses a string containing JSONC to an AST with comments and tokens.
///
/// # Example
///
/// ```
/// use jsonc_parser::{parse_to_ast, ParseOptions};
///
/// let parse_result = parse_to_ast(r#"{ "test": 5 } // test"#, &ParseOptions {
///     comments: true, // include comments in result
///     tokens: true, // include tokens in result
/// }).expect("Should parse.");
/// // ...inspect parse_result for value, tokens, and comments here...
/// ```
pub fn parse_to_ast<'a>(text: &'a str, options: &ParseOptions) -> Result<ParseResult<'a>, ParseError> {
    let mut context = Context {
        scanner: Scanner::new(text),
        comments: if options.comments { Some(HashMap::new()) } else { None },
        current_comments: None,
        last_token_end: 0,
        range_stack: Vec::new(),
        tokens: if options.tokens { Some(Vec::new()) } else { None },
    };
    context.scan()?;
    let value = parse_value(&mut context)?;

    if context.scan()?.is_some() {
        return Err(context.create_parse_error("Text cannot contain more than one JSON value"));
    }

    debug_assert!(context.range_stack.is_empty());

    Ok(ParseResult {
        comments: context.comments,
        tokens: context.tokens,
        value,
    })
}

fn parse_value<'a>(context: &mut Context<'a>) -> Result<Option<Value<'a>>, ParseError> {
    match context.token() {
        None => Ok(None),
        Some(token) => match token {
            Token::OpenBrace => Ok(Some(Value::Object(parse_object(context)?))),
            Token::OpenBracket => Ok(Some(Value::Array(parse_array(context)?))),
            Token::String(value) => Ok(Some(Value::StringLit(create_string_lit(context, value)))),
            Token::Boolean(value) => Ok(Some(Value::BooleanLit(create_boolean_lit(context, value)))),
            Token::Number(value) => Ok(Some(Value::NumberLit(create_number_lit(context, value)))),
            Token::Null => return Ok(Some(Value::NullKeyword(create_null_keyword(context)))),
            Token::CloseBracket => Err(context.create_parse_error("Unexpected close bracket")),
            Token::CloseBrace => Err(context.create_parse_error("Unexpected close brace")),
            Token::Comma => Err(context.create_parse_error("Unexpected comma")),
            Token::Colon => Err(context.create_parse_error("Unexpected colon")),
            Token::Word(_) => Err(context.create_parse_error("Unexpected word")),
            Token::CommentLine(_) => unreachable!(),
            Token::CommentBlock(_) => unreachable!(),
        },
    }
}

fn parse_object<'a>(context: &mut Context<'a>) -> Result<Object<'a>, ParseError> {
    debug_assert!(context.token() == Some(Token::OpenBrace));
    let mut properties = Vec::new();

    context.start_range();
    context.scan()?;

    loop {
        match context.token() {
            Some(Token::CloseBrace) => break,
            Some(Token::String(prop_name)) => {
                properties.push(parse_object_property(context, PropName::String(prop_name))?);
            }
            Some(Token::Word(prop_name)) | Some(Token::Number(prop_name)) => {
                properties.push(parse_object_property(context, PropName::Word(prop_name))?);
            }
            None => return Err(context.create_parse_error_for_current_range("Unterminated object")),
            _ => return Err(context.create_parse_error("Unexpected token in object")),
        }

        // skip the comma
        if let Some(Token::Comma) = context.scan()? {
            context.scan()?;
        }
    }

    Ok(Object {
        range: context.end_range(),
        properties,
    })
}

enum PropName<'a> {
    String(Cow<'a, str>),
    Word(&'a str),
}

fn parse_object_property<'a>(context: &mut Context<'a>, prop_name: PropName<'a>) -> Result<ObjectProp<'a>, ParseError> {
    context.start_range();

    let name = match prop_name {
        PropName::String(prop_name) => ObjectPropName::String(create_string_lit(context, prop_name)),
        PropName::Word(prop_name) => ObjectPropName::Word(create_word(context, prop_name)),
    };

    match context.scan()? {
        Some(Token::Colon) => {}
        _ => return Err(context.create_parse_error("Expected a colon after the string or word in an object property")),
    }

    context.scan()?;
    let value = parse_value(context)?;

    match value {
        Some(value) => Ok(ObjectProp {
            range: context.end_range(),
            name,
            value,
        }),
        None => Err(context.create_parse_error("Expected value after colon in object property")),
    }
}

fn parse_array<'a>(context: &mut Context<'a>) -> Result<Array<'a>, ParseError> {
    debug_assert!(context.token() == Some(Token::OpenBracket));
    let mut elements = Vec::new();

    context.start_range();
    context.scan()?;

    loop {
        match context.token() {
            Some(Token::CloseBracket) => break,
            None => return Err(context.create_parse_error_for_current_range("Unterminated array")),
            _ => match parse_value(context)? {
                Some(value) => elements.push(value),
                None => return Err(context.create_parse_error_for_current_range("Unterminated array")),
            },
        }

        // skip the comma
        if let Some(Token::Comma) = context.scan()? {
            context.scan()?;
        }
    }

    Ok(Array {
        range: context.end_range(),
        elements,
    })
}

// factory functions

fn create_string_lit<'a>(context: &Context<'a>, value: Cow<'a, str>) -> StringLit<'a> {
    StringLit {
        range: context.create_range_from_last_token(),
        value,
    }
}

fn create_word<'a>(context: &Context<'a>, value: &'a str) -> WordLit<'a> {
    WordLit {
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

fn create_number_lit<'a>(context: &Context<'a>, value: &'a str) -> NumberLit<'a> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_error_when_has_multiple_values() {
        assert_has_error(
            "[][]",
            "Text cannot contain more than one JSON value on line 1 column 3.",
        );
    }

    #[test]
    fn it_should_error_when_object_is_not_terminated() {
        assert_has_error("{", "Unterminated object on line 1 column 1.");
    }

    #[test]
    fn it_should_error_when_object_has_unexpected_token() {
        assert_has_error("{ [] }", "Unexpected token in object on line 1 column 3.");
    }

    #[test]
    fn it_should_error_when_object_has_two_non_string_tokens() {
        assert_has_error(
            "{ asdf asdf: 5 }",
            "Expected a colon after the string or word in an object property on line 1 column 8.",
        );
    }

    #[test]
    fn it_should_error_when_array_is_not_terminated() {
        assert_has_error("[", "Unterminated array on line 1 column 1.");
    }

    #[test]
    fn it_should_error_when_array_has_unexpected_token() {
        assert_has_error("[:]", "Unexpected colon on line 1 column 2.");
    }

    #[test]
    fn it_should_error_when_comment_block_not_closed() {
        assert_has_error("/* test", "Unterminated comment block on line 1 column 1.");
    }

    #[test]
    fn it_should_error_when_string_lit_not_closed() {
        assert_has_error("\" test", "Unterminated string literal on line 1 column 1.");
    }

    fn assert_has_error(text: &str, message: &str) {
        let result = parse_to_ast(text, &Default::default());
        match result {
            Ok(_) => panic!("Expected error, but did not find one."),
            Err(err) => assert_eq!(err.to_string(), message),
        }
    }

    #[test]
    fn it_should_not_include_tokens_by_default() {
        let result = parse_to_ast("{}", &Default::default()).unwrap();
        assert_eq!(result.tokens.is_none(), true);
    }

    #[test]
    fn it_should_include_tokens_when_specified() {
        let result = parse_to_ast(
            "{}",
            &ParseOptions {
                tokens: true,
                ..Default::default()
            },
        )
        .unwrap();
        let tokens = result.tokens.unwrap();
        assert_eq!(tokens.len(), 2);
    }

    #[test]
    fn it_should_not_include_comments_by_default() {
        let result = parse_to_ast("{}", &Default::default()).unwrap();
        assert_eq!(result.comments.is_none(), true);
    }

    #[test]
    fn it_should_include_comments_when_specified() {
        let result = parse_to_ast(
            "{} // 2",
            &ParseOptions {
                comments: true,
                ..Default::default()
            },
        )
        .unwrap();
        let comments = result.comments.unwrap();
        assert_eq!(comments.len(), 2); // for both positions, but it's the same comment
    }
}
