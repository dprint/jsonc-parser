use super::common::Range;
use super::errors::*;
use super::tokens::Token;
use std::borrow::Cow;
use std::str::Chars;

/// Converts text into a stream of tokens.
pub struct Scanner<'a> {
    byte_index: usize,
    line_number: usize,
    token_start: usize,
    token_start_line: usize,
    char_iter: Chars<'a>,
    char_buffer: Vec<char>,
    current_token: Option<Token<'a>>,
    file_text: &'a str,
}

const CHAR_BUFFER_MAX_SIZE: usize = 6;

impl<'a> Scanner<'a> {
    /// Creates a new scanner based on the provided text.
    pub fn new(text: &'a str) -> Scanner<'a> {
        let mut char_iter = text.chars();
        let mut char_buffer = Vec::with_capacity(CHAR_BUFFER_MAX_SIZE);
        let current_char = char_iter.next();
        if let Some(current_char) = current_char {
            char_buffer.push(current_char);
        }

        Scanner {
            byte_index: 0,
            line_number: 0,
            token_start: 0,
            token_start_line: 0,
            char_iter,
            char_buffer,
            current_token: None,
            file_text: text,
        }
    }

    /// Moves to and returns the next token.
    pub fn scan(&mut self) -> Result<Option<Token<'a>>, ParseError> {
        self.skip_whitespace();
        self.token_start = self.byte_index;
        self.token_start_line = self.line_number;
        if let Some(current_char) = self.current_char() {
            let token_result = match current_char {
                '{' => {
                    self.move_next_char();
                    Ok(Token::OpenBrace)
                }
                '}' => {
                    self.move_next_char();
                    Ok(Token::CloseBrace)
                }
                '[' => {
                    self.move_next_char();
                    Ok(Token::OpenBracket)
                }
                ']' => {
                    self.move_next_char();
                    Ok(Token::CloseBracket)
                }
                ',' => {
                    self.move_next_char();
                    Ok(Token::Comma)
                }
                ':' => {
                    self.move_next_char();
                    Ok(Token::Colon)
                }
                '\'' | '"' => self.parse_string(),
                '/' => match self.peek_char() {
                    Some('/') => Ok(self.parse_comment_line()),
                    Some('*') => self.parse_comment_block(),
                    _ => Err(self.create_error_for_current_token("Unexpected token")),
                },
                _ => {
                    if current_char == '-' || self.is_digit() {
                        self.parse_number()
                    } else if self.try_move_word("true") {
                        Ok(Token::Boolean(true))
                    } else if self.try_move_word("false") {
                        Ok(Token::Boolean(false))
                    } else if self.try_move_word("null") {
                        Ok(Token::Null)
                    } else {
                        self.parse_word()
                    }
                }
            };
            match token_result {
                Ok(token) => {
                    self.current_token = Some(token.clone());
                    Ok(Some(token))
                }
                Err(err) => Err(err),
            }
        } else {
            self.current_token = None;
            Ok(None)
        }
    }

    /// Gets the start position of the token.
    pub fn token_start(&self) -> usize {
        self.token_start
    }

    /// Gets the end position of the token.
    pub fn token_end(&self) -> usize {
        self.byte_index
    }

    /// Gets the line the token starts on.
    pub fn token_start_line(&self) -> usize {
        self.token_start_line
    }

    /// Gets the line the token ends on.
    pub fn token_end_line(&self) -> usize {
        self.line_number
    }

    /// Gets the current token.
    pub fn token(&self) -> Option<Token<'a>> {
        self.current_token.as_ref().map(|x| x.to_owned())
    }

    pub(super) fn create_error_for_current_token(&self, message: &str) -> ParseError {
        self.create_error_for_start_and_line(self.token_start, self.token_start_line, message)
    }

    pub(super) fn create_error_for_current_char(&self, message: &str) -> ParseError {
        self.create_error_for_start_and_line(self.byte_index, self.line_number, message)
    }

    pub(super) fn create_error_for_start_and_line(
        &self,
        start: usize,
        start_line: usize,
        message: &str,
    ) -> ParseError {
        let range = Range {
            start,
            start_line,
            end: std::cmp::min(self.byte_index + 1, self.file_text.len()),
            end_line: self.line_number,
        };
        self.create_error_for_range(range, message)
    }

    pub(super) fn create_error_for_range(&self, range: Range, message: &str) -> ParseError {
        ParseError::new(range, message, &self.file_text)
    }

    fn parse_string(&mut self) -> Result<Token<'a>, ParseError> {
        debug_assert!(
            self.current_char() == Some('\'') || self.current_char() == Some('"'),
            "Expected \", was {:?}",
            self.current_char()
        );
        let is_double_quote = self.current_char() == Some('"');
        let mut last_start_byte_index = self.byte_index + 1;
        let mut text: Option<String> = None;
        let mut last_was_backslash = false;
        let mut found_end_string = false;

        while let Some(current_char) = self.move_next_char() {
            if last_was_backslash {
                let escape_start = self.byte_index - 1; // -1 for backslash
                let escape_start_line = self.line_number;
                match current_char {
                    '"' | '\'' | '\\' | '/' | 'b' | 'f' | 'u' | 'r' | 'n' | 't' => {
                        if current_char == '"' {
                            if !is_double_quote {
                                return Err(self.create_error_for_start_and_line(
                                    escape_start,
                                    escape_start_line,
                                    "Invalid escape in single quote string",
                                ));
                            }
                        } else if current_char == '\'' {
                            if is_double_quote {
                                return Err(self.create_error_for_start_and_line(
                                    escape_start,
                                    escape_start_line,
                                    "Invalid escape in double quote string",
                                ));
                            }
                        }

                        let previous_text = &self.file_text[last_start_byte_index..escape_start];
                        if text.is_none() {
                            text = Some(String::new());
                        }
                        let text = text.as_mut().unwrap();
                        text.push_str(previous_text);
                        if current_char == 'u' {
                            let mut hex_text = String::new();
                            // expect four hex values
                            for _ in 0..4 {
                                let current_char = self.move_next_char();
                                if !self.is_hex() {
                                    return Err(self.create_error_for_start_and_line(
                                        escape_start,
                                        escape_start_line,
                                        "Expected four hex digits",
                                    ));
                                }
                                if let Some(current_char) = current_char {
                                    hex_text.push(current_char);
                                }
                            }

                            let hex_u32 = u32::from_str_radix(&hex_text, 16);
                            let hex_char = match hex_u32
                                .ok()
                                .map(|hex_u32| std::char::from_u32(hex_u32))
                                .flatten()
                            {
                                Some(hex_char) => hex_char,
                                None => {
                                    return Err(self.create_error_for_start_and_line(
                                        escape_start,
                                        escape_start_line,
                                        &format!(
                                            "Invalid unicode escape sequence. '{}' is not a valid UTF8 character",
                                            hex_text
                                        ),
                                    ));
                                }
                            };
                            text.push(hex_char);
                            last_start_byte_index = self.byte_index
                                + self.current_char().map(|c| c.len_utf8()).unwrap_or(0);
                        } else {
                            text.push(match current_char {
                                'b' => '\u{08}',
                                'f' => '\u{0C}',
                                't' => '\t',
                                'r' => '\r',
                                'n' => '\n',
                                _ => current_char,
                            });
                            last_start_byte_index = self.byte_index + current_char.len_utf8();
                        }
                    }
                    _ => {
                        return Err(self.create_error_for_start_and_line(
                            escape_start,
                            escape_start_line,
                            "Invalid escape",
                        ))
                    }
                }
                last_was_backslash = false;
            } else if is_double_quote && current_char == '"'
                || !is_double_quote && current_char == '\''
            {
                found_end_string = true;
                break;
            } else {
                last_was_backslash = current_char == '\\';
            }
        }

        if found_end_string {
            self.move_next_char();
            let final_segment = &self.file_text[last_start_byte_index..self.byte_index - 1];
            Ok(Token::String(match text {
                Some(mut text) => {
                    text.push_str(final_segment);
                    Cow::Owned(text)
                }
                None => Cow::Borrowed(final_segment),
            }))
        } else {
            Err(self.create_error_for_current_token("Unterminated string literal"))
        }
    }

    fn parse_number(&mut self) -> Result<Token<'a>, ParseError> {
        let start_byte_index = self.byte_index;

        if self.is_negative_sign() {
            self.move_next_char();
        }

        if self.is_zero() {
            self.move_next_char();
        } else if self.is_one_nine() {
            self.move_next_char();
            while self.is_digit() {
                self.move_next_char();
            }
        } else {
            return Err(
                self.create_error_for_current_char("Expected a digit to follow a negative sign")
            );
        }

        if self.is_decimal_point() {
            self.move_next_char();

            if !self.is_digit() {
                return Err(self.create_error_for_current_char("Expected a digit"));
            }

            while self.is_digit() {
                self.move_next_char();
            }
        }

        match self.current_char() {
            Some('e') | Some('E') => match self.move_next_char() {
                Some('-') | Some('+') => {
                    self.move_next_char();
                    if !self.is_digit() {
                        return Err(self.create_error_for_current_char("Expected a digit"));
                    }
                    while self.is_digit() {
                        self.move_next_char();
                    }
                }
                _ => {
                    return Err(self.create_error_for_current_char(
                        "Expected plus or minus symbol in number literal",
                    ));
                }
            },
            _ => {}
        }

        let end_byte_index = self.byte_index;
        Ok(Token::Number(
            &self.file_text[start_byte_index..end_byte_index],
        ))
    }

    fn parse_comment_line(&mut self) -> Token<'a> {
        self.assert_then_move_char('/');
        #[cfg(debug_assertions)]
        self.assert_char('/');

        let start_byte_index = self.byte_index + 1;
        while let Some(_) = self.move_next_char() {
            if self.is_new_line() {
                break;
            }
        }

        Token::CommentLine(&self.file_text[start_byte_index..self.byte_index])
    }

    fn parse_comment_block(&mut self) -> Result<Token<'a>, ParseError> {
        self.assert_then_move_char('/');
        #[cfg(debug_assertions)]
        self.assert_char('*');
        let mut found_end = false;

        let start_byte_index = self.byte_index + 1;
        while let Some(current_char) = self.move_next_char() {
            if current_char == '*' && self.peek_char() == Some('/') {
                found_end = true;
                break;
            }
        }

        if found_end {
            let end_byte_index = self.byte_index;
            self.assert_then_move_char('*');
            self.assert_then_move_char('/');
            Ok(Token::CommentBlock(
                &self.file_text[start_byte_index..end_byte_index],
            ))
        } else {
            Err(self.create_error_for_current_token("Unterminated comment block"))
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(current_char) = self.current_char() {
            if current_char.is_whitespace() {
                self.move_next_char();
            } else {
                break;
            }
        }
    }

    fn try_move_word(&mut self, text: &str) -> bool {
        let mut char_index = 0;
        for c in text.chars() {
            if let Some(current_char) = self.peek_char_offset(char_index) {
                if current_char != c {
                    return false;
                }

                char_index += 1;
            } else {
                return false;
            }
        }

        if let Some(next_char) = self.peek_char_offset(char_index) {
            if next_char.is_alphanumeric() {
                return false;
            }
        }

        for _ in 0..char_index {
            self.move_next_char();
        }

        true
    }

    fn parse_word(&mut self) -> Result<Token<'a>, ParseError> {
        let start_byte_index = self.byte_index;

        while let Some(current_char) = self.current_char() {
            if current_char.is_whitespace()
                || current_char == '\r'
                || current_char == '\n'
                || current_char == ':'
            {
                break;
            }
            if !current_char.is_alphanumeric() && current_char != '-' {
                return Err(self.create_error_for_current_token("Unexpected token"));
            }

            self.move_next_char();
        }

        let end_byte_index = self.byte_index;

        if end_byte_index - start_byte_index == 0 {
            return Err(self.create_error_for_current_token("Unexpected token"));
        }

        Ok(Token::Word(
            &self.file_text[start_byte_index..end_byte_index],
        ))
    }

    fn assert_then_move_char(&mut self, _character: char) {
        #[cfg(debug_assertions)]
        self.assert_char(_character);

        self.move_next_char();
    }

    #[cfg(debug_assertions)]
    fn assert_char(&mut self, character: char) {
        let current_char = self.current_char();
        debug_assert!(
            current_char == Some(character),
            "Expected {:?}, was {:?}",
            character,
            current_char
        );
    }

    fn move_next_char(&mut self) -> Option<char> {
        if let Some(&current_char) = self.char_buffer.get(0) {
            if current_char == '\n' {
                self.line_number += 1;
            }

            // shift the entire array to the left then pop the last item
            for i in 1..self.char_buffer.len() {
                self.char_buffer[i - 1] = self.char_buffer[i];
            }
            self.char_buffer.pop();

            if self.char_buffer.is_empty() {
                if let Some(new_char) = self.char_iter.next() {
                    self.char_buffer.push(new_char);
                }
            }

            self.byte_index += current_char.len_utf8();
        }

        self.current_char()
    }

    fn peek_char(&mut self) -> Option<char> {
        self.peek_char_offset(1)
    }

    fn peek_char_offset(&mut self, offset: usize) -> Option<char> {
        // fill the char buffer
        for _ in self.char_buffer.len()..offset + 1 {
            if let Some(next_char) = self.char_iter.next() {
                self.char_buffer.push(next_char);
            } else {
                // end of string
                return None;
            }
        }

        // should not exceed this
        debug_assert!(self.char_buffer.len() <= CHAR_BUFFER_MAX_SIZE);

        self.char_buffer.get(offset).map(|c| *c)
    }

    fn current_char(&self) -> Option<char> {
        self.char_buffer.get(0).map(|c| *c)
    }

    fn is_new_line(&mut self) -> bool {
        match self.current_char() {
            Some('\n') => true,
            Some('\r') => self.peek_char() == Some('\n'),
            _ => false,
        }
    }

    fn is_hex(&self) -> bool {
        self.is_digit()
            || match self.current_char() {
                Some(current_char) => {
                    current_char >= 'a' && current_char <= 'f'
                        || current_char >= 'A' && current_char <= 'F'
                }
                _ => false,
            }
    }

    fn is_digit(&self) -> bool {
        self.is_one_nine() || self.is_zero()
    }

    fn is_zero(&self) -> bool {
        self.current_char() == Some('0')
    }

    fn is_one_nine(&self) -> bool {
        match self.current_char() {
            Some(current_char) => current_char >= '1' && current_char <= '9',
            _ => false,
        }
    }

    fn is_negative_sign(&self) -> bool {
        self.current_char() == Some('-')
    }

    fn is_decimal_point(&self) -> bool {
        self.current_char() == Some('.')
    }
}

#[cfg(test)]
mod tests {
    use super::super::tokens::Token;
    use super::*;

    #[test]
    fn it_tokenizes_string() {
        assert_has_tokens(
            r#""t\"est", "\t\r\n\n\u0020 test\n other","#,
            vec![
                Token::String(Cow::Borrowed(r#"t"est"#)),
                Token::Comma,
                Token::String(Cow::Borrowed("\t\r\n\n  test\n other")),
                Token::Comma,
            ],
        );
    }

    #[test]
    fn it_errors_escaping_single_quote_in_double_quote() {
        assert_has_error(
            r#""t\'est""#,
            "Invalid escape in double quote string on line 1 column 3.",
        );
    }

    #[test]
    fn it_tokenizes_single_quote_string() {
        assert_has_tokens(
            r#"'t\'est','a',"#,
            vec![
                Token::String(Cow::Borrowed(r#"t'est"#)),
                Token::Comma,
                Token::String(Cow::Borrowed("a")),
                Token::Comma,
            ],
        );
    }

    #[test]
    fn it_errors_escaping_double_quote_in_single_quote() {
        assert_has_error(
            r#"'t\"est'"#,
            "Invalid escape in single quote string on line 1 column 3.",
        );
    }

    #[test]
    fn it_errors_for_word_starting_with_invalid_token() {
        assert_has_error(r#"{ &test }"#, "Unexpected token on line 1 column 3.");
    }

    #[test]
    fn it_tokenizes_numbers() {
        assert_has_tokens(
            "0, 0.123, -198, 0e-345, 0.3e+025,",
            vec![
                Token::Number("0"),
                Token::Comma,
                Token::Number("0.123"),
                Token::Comma,
                Token::Number("-198"),
                Token::Comma,
                Token::Number("0e-345"),
                Token::Comma,
                Token::Number("0.3e+025"),
                Token::Comma,
            ],
        );
    }

    #[test]
    fn it_tokenizes_simple_tokens() {
        assert_has_tokens(
            "{}[],:true,false,null,",
            vec![
                Token::OpenBrace,
                Token::CloseBrace,
                Token::OpenBracket,
                Token::CloseBracket,
                Token::Comma,
                Token::Colon,
                Token::Boolean(true),
                Token::Comma,
                Token::Boolean(false),
                Token::Comma,
                Token::Null,
                Token::Comma,
            ],
        );
    }

    #[test]
    fn it_tokenizes_comment_line() {
        assert_has_tokens(
            "//test\n//t\r\n// test\n,",
            vec![
                Token::CommentLine("test"),
                Token::CommentLine("t"),
                Token::CommentLine(" test"),
                Token::Comma,
            ],
        );
    }

    #[test]
    fn it_tokenizes_comment_blocks() {
        assert_has_tokens(
            "/*test\n *//* test*/,",
            vec![
                Token::CommentBlock("test\n "),
                Token::CommentBlock(" test"),
                Token::Comma,
            ],
        );
    }

    #[test]
    fn it_errors_on_invalid_utf8_char_for_issue_6() {
        assert_has_error(
            "\"\\uDF06\"",
            "Invalid unicode escape sequence. 'DF06' is not a valid UTF8 character on line 1 column 2.",
        );
    }

    fn assert_has_tokens(text: &str, tokens: Vec<Token>) {
        let mut scanner = Scanner::new(text);
        let mut scanned_tokens = Vec::new();

        loop {
            match scanner.scan() {
                Ok(Some(token)) => scanned_tokens.push(token),
                Ok(None) => break,
                Err(err) => panic!("Error parsing: {:?}", err),
            }
        }

        assert_eq!(scanned_tokens, tokens);
    }

    fn assert_has_error(text: &str, message: &str) {
        let mut scanner = Scanner::new(text);
        let mut error_message = String::new();

        loop {
            match scanner.scan() {
                Ok(Some(_)) => {}
                Ok(None) => break,
                Err(err) => {
                    error_message = err.to_string();
                    break;
                }
            }
        }

        assert_eq!(error_message, message);
    }
}
