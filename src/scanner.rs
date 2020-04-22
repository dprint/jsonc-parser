use super::tokens::Token;
use super::errors::*;
use super::common::ImmutableString;

pub struct Scanner {
    pos: usize,
    line_number: usize,
    token_start: usize,
    token_start_line: usize,
    chars: Vec<char>, // todo: use an iterator instead?
    current_token: Option<Token>,
}

impl Scanner {
    pub fn new(text: &str) -> Scanner {
        Scanner {
            pos: 0,
            line_number: 0,
            token_start: 0,
            token_start_line: 0,
            chars: text.chars().collect(),
            current_token: None,
        }
    }

    pub fn scan(&mut self) -> Result<Option<Token>, ParseError> {
        self.skip_whitespace();
        self.token_start = self.pos;
        self.token_start_line = self.line_number;
        if let Some(current_char) = self.current_char() {
            let token_result = match current_char {
                '{' => {
                    self.move_next_char();
                    Ok(Token::OpenBrace)
                },
                '}' => {
                    self.move_next_char();
                    Ok(Token::CloseBrace)
                },
                '[' => {
                    self.move_next_char();
                    Ok(Token::OpenBracket)
                },
                ']' => {
                    self.move_next_char();
                    Ok(Token::CloseBracket)
                },
                ',' => {
                    self.move_next_char();
                    Ok(Token::Comma)
                },
                ':' => {
                    self.move_next_char();
                    Ok(Token::Colon)
                },
                '"' => self.parse_string(),
                '/' => {
                    match self.peek_char() {
                        Some('/') => Ok(self.parse_comment_line()),
                        Some('*') => self.parse_comment_block(),
                        _ => Err(ParseError::new(self.token_start, "Unexpected token.")),
                    }
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
                        Err(ParseError::new(self.token_start, "Unexpected token."))
                    }
                }
            };
            match token_result {
                Ok(token) => {
                    self.current_token = Some(token.clone());
                    Ok(Some(token))
                },
                Err(err) => Err(err),
            }
        } else {
            self.current_token = None;
            Ok(None)
        }
    }

    pub fn token_start(&self) -> usize {
        self.token_start
    }

    pub fn token_end(&self) -> usize {
        self.pos
    }

    pub fn token_start_line(&self) -> usize {
        self.token_start_line
    }

    pub fn token_end_line(&self) -> usize {
        self.line_number
    }

    pub fn token(&self) -> Option<Token> {
        self.current_token.as_ref().map(|x| x.to_owned())
    }

    fn parse_string(&mut self) -> Result<Token, ParseError> {
        #[cfg(debug_assertions)]
        self.assert_char('"');
        let start_pos = self.pos;
        let mut text = String::new();
        let mut last_was_backslash = false;
        let mut found_end_string = false;

        while let Some(current_char) = self.move_next_char() {
            if last_was_backslash {
                match current_char {
                    '"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't' => {
                        text.push(current_char);
                    },
                    'u' => {
                        text.push(current_char);
                        let hex_start_pos = self.pos - 1;
                        // expect four hex values
                        for _ in 0..4 {
                            if let Some(current_char) = self.move_next_char() {
                                text.push(current_char);
                            }
                            if !self.is_hex() {
                                return Err(ParseError::new(hex_start_pos, "Expected four hex digits."));
                            }
                        }
                    },
                    _ => return Err(ParseError::new(start_pos, "Invalid escape.")),
                }
                last_was_backslash = false;
            } else if current_char == '"' {
                found_end_string = true;
                break;
            } else {
                last_was_backslash = current_char == '\\';
                text.push(current_char);
            }
        }

        if found_end_string {
            self.move_next_char();
            Ok(Token::String(ImmutableString::new(text)))
        } else {
            Err(ParseError::new(start_pos, "Unterminated string literal"))
        }
    }

    fn parse_number(&mut self) -> Result<Token, ParseError> {
        let mut text = String::new();

        if self.is_negative_sign() {
            text.push('-');
            self.move_next_char();
        }

        if self.is_zero() {
            text.push('0');
            self.move_next_char();
        } else if self.is_one_nine() {
            text.push(self.current_char().unwrap());
            self.move_next_char();
            while self.is_digit() {
                text.push(self.current_char().unwrap());
                self.move_next_char();
            }
        } else {
            return Err(ParseError::new(self.pos, "Expected a digit to follow a negative sign."));
        }

        if self.is_decimal_point() {
            text.push('.');
            self.move_next_char();

            if !self.is_digit() {
                return Err(ParseError::new(self.pos, "Expected a digit."));
            }

            while self.is_digit() {
                text.push(self.current_char().unwrap());
                self.move_next_char();
            }
        }

        match self.current_char() {
            Some('e') | Some('E') => {
                text.push(self.current_char().unwrap());
                match self.move_next_char() {
                    Some('-') | Some('+') => {
                        text.push(self.current_char().unwrap());
                        self.move_next_char();
                        if !self.is_digit() {
                            return Err(ParseError::new(self.pos, "Expected a digit."));
                        }
                        while self.is_digit() {
                            text.push(self.current_char().unwrap());
                            self.move_next_char();
                        }
                    }
                    _ => {
                        return Err(ParseError::new(self.pos, "Expected plus or minus symbol in number literal."));
                    }
                }
            }
            _ => {},
        }


        Ok(Token::Number(ImmutableString::new(text)))
    }

    fn parse_comment_line(&mut self) -> Token {
        let mut text = String::new();
        self.assert_then_move_char('/');
        #[cfg(debug_assertions)]
        self.assert_char('/');

        while let Some(current_char) = self.move_next_char() {
            if self.is_new_line() {
                break;
            }
            text.push(current_char);
        }

        Token::CommentLine(ImmutableString::new(text))
    }

    fn parse_comment_block(&mut self) -> Result<Token, ParseError> {
        let token_start = self.pos;
        let mut text = String::new();
        self.assert_then_move_char('/');
        #[cfg(debug_assertions)]
        self.assert_char('*');
        let mut found_end = false;

        while let Some(current_char) = self.move_next_char() {
            if current_char == '*' && self.peek_char() == Some('/') {
                found_end = true;
                break;
            }
            text.push(current_char);
        }

        if found_end {
            self.assert_then_move_char('*');
            self.assert_then_move_char('/');
            Ok(Token::CommentBlock(ImmutableString::new(text)))
        } else {
            Err(ParseError::new(token_start, "Unterminated comment block."))
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
        // todo: debug assert no newlines
        let mut i = self.pos;
        for c in text.chars() {
            if let Some(current_char) = self.chars.get(i) {
                if *current_char != c {
                    return false;
                }
            } else {
                return false;
            }
            i += 1;
        }

        if let Some(next_char) = self.chars.get(i) {
            if next_char.is_alphanumeric() {
                return false;
            }
        }

        self.pos = i;
        true
    }

    fn assert_then_move_char(&mut self, character: char) {
        #[cfg(debug_assertions)]
        self.assert_char(character);

        self.move_next_char();
    }

    #[cfg(debug_assertions)]
    fn assert_char(&mut self, character: char) {
        let current_char = self.current_char();
        debug_assert!(current_char == Some(character), "Expected {:?}, was {:?}", character, current_char);
    }

    fn move_next_char(&mut self) -> Option<char> {
        if self.current_char() == Some('\n') {
            self.line_number += 1;
        }
        self.pos += 1;
        self.current_char()
    }

    fn peek_char(&self) -> Option<char> {
        self.chars.get(self.pos + 1).map(|x| x.to_owned())
    }

    fn current_char(&self) -> Option<char> {
        self.chars.get(self.pos).map(|x| x.to_owned())
    }

    fn is_new_line(&self) -> bool {
        match self.current_char() {
            Some('\n') => true,
            Some('\r') => self.peek_char() == Some('\n'),
            _ => false,
        }
    }

    fn is_hex(&self) -> bool {
        self.is_digit() || match self.current_char() {
            Some(current_char) => current_char >= 'a' && current_char <= 'f'
                || current_char >= 'A' && current_char <= 'F',
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
    use super::Scanner;
    use super::super::common::{ImmutableString};
    use super::super::tokens::{Token};

    #[test]
    fn it_tokenizes_string() {
        assert_has_tokens(
            r#""t\"est", "\r\n\n\ua0B9","#,
            vec![
                Token::String(ImmutableString::from(r#"t\"est"#)),
                Token::Comma,
                Token::String(ImmutableString::from("\\r\\n\\n\\ua0B9")),
                Token::Comma,
            ]
        );
    }

    #[test]
    fn it_tokenizes_numbers() {
        assert_has_tokens(
            "0, 0.123, -198, 0e-345, 0.3e+025,",
            vec![
                Token::Number(ImmutableString::from("0")),
                Token::Comma,
                Token::Number(ImmutableString::from("0.123")),
                Token::Comma,
                Token::Number(ImmutableString::from("-198")),
                Token::Comma,
                Token::Number(ImmutableString::from("0e-345")),
                Token::Comma,
                Token::Number(ImmutableString::from("0.3e+025")),
                Token::Comma,
            ]
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
            ]);
    }

    #[test]
    fn it_tokenizes_comment_line() {
        assert_has_tokens(
            "//test\n//t\r\n// test\n,",
            vec![
                Token::CommentLine(ImmutableString::from("test")),
                Token::CommentLine(ImmutableString::from("t")),
                Token::CommentLine(ImmutableString::from(" test")),
                Token::Comma,
            ]);
    }

    #[test]
    fn it_tokenizes_comment_blocks() {
        assert_has_tokens(
            "/*test\n *//* test*/,",
            vec![
                Token::CommentBlock(ImmutableString::from("test\n ")),
                Token::CommentBlock(ImmutableString::from(" test")),
                Token::Comma,
            ]);
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
}
