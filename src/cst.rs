use std::collections::VecDeque;
use std::fmt::Display;

use super::common::Range;
use super::common::Ranged;
use crate::ast;
use crate::errors::ParseError;
use crate::parse_to_ast;
use crate::ParseOptions;

pub struct CstRootNode {
  children: Vec<CstNode>,
}

impl CstRootNode {
  pub fn parse(text: &str, parse_options: &ParseOptions) -> Result<Self, ParseError> {
    let parse_result = parse_to_ast(
      text,
      &crate::CollectOptions {
        comments: crate::CommentCollectionStrategy::AsTokens,
        tokens: true,
      },
      parse_options,
    )?;

    // turn the AST into a CST
    Ok(
      CstBuilder {
        text,
        tokens: parse_result.tokens.unwrap().into_iter().collect(),
      }
      .build(parse_result.value),
    )
  }
}

impl Display for CstRootNode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for child in &self.children {
      write!(f, "{}", child)?;
    }
    Ok(())
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CstNode {
  StringLit(CstStringLit),
  NumberLit(CstNumberLit),
  BooleanLit(CstBooleanLit),
  Object(CstObject),
  ObjectProp(CstObjectProp),
  Array(CstArray),
  NullKeyword(CstNullKeyword),
  WordLit(CstWordLit),
  Token(CstToken),
  Whitespace(CstWhitespace),
  Comment(CstComment),
}

impl Display for CstNode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      CstNode::StringLit(node) => write!(f, "{}", node),
      CstNode::NumberLit(node) => write!(f, "{}", node),
      CstNode::BooleanLit(node) => write!(f, "{}", node),
      CstNode::Object(node) => write!(f, "{}", node),
      CstNode::ObjectProp(node) => write!(f, "{}", node),
      CstNode::Array(node) => write!(f, "{}", node),
      CstNode::NullKeyword(node) => write!(f, "{}", node),
      CstNode::WordLit(node) => write!(f, "{}", node),
      CstNode::Token(node) => write!(f, "{}", node),
      CstNode::Whitespace(node) => write!(f, "{}", node),
      CstNode::Comment(node) => write!(f, "{}", node),
    }
  }
}

impl Ranged for CstNode {
  fn range(&self) -> Range {
    match self {
      CstNode::StringLit(node) => node.range,
      CstNode::NumberLit(node) => node.range,
      CstNode::BooleanLit(node) => node.range,
      CstNode::Object(node) => node.range,
      CstNode::ObjectProp(node) => node.range,
      CstNode::Array(node) => node.range,
      CstNode::NullKeyword(node) => node.range,
      CstNode::WordLit(node) => node.range,
      CstNode::Token(node) => node.range,
      CstNode::Whitespace(node) => node.range,
      CstNode::Comment(node) => node.range,
    }
  }
}

/// Node surrounded in double quotes (ex. `"my string"`).
#[derive(Debug, PartialEq, Clone)]
pub struct CstStringLit {
  pub range: Range,
  pub value: String,
}

impl Display for CstStringLit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "\"{}\"", self.value)
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CstWordLit {
  pub range: Range,
  pub value: String,
}

impl Display for CstWordLit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.value)
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CstNumberLit {
  pub range: Range,
  pub value: String,
}

impl Display for CstNumberLit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.value)
  }
}

/// Represents a boolean (ex. `true` or `false`).
#[derive(Debug, PartialEq, Clone)]
pub struct CstBooleanLit {
  pub range: Range,
  pub value: bool,
}

impl Display for CstBooleanLit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.value {
      write!(f, "true")
    } else {
      write!(f, "false")
    }
  }
}

/// Represents the null keyword (ex. `null`).
#[derive(Debug, PartialEq, Clone)]
pub struct CstNullKeyword {
  pub range: Range,
}

impl Display for CstNullKeyword {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "null")
  }
}

/// Represents an object that may contain properties (ex. `{}`, `{ "prop": 4 }`).
#[derive(Debug, PartialEq, Clone)]
pub struct CstObject {
  pub range: Range,
  pub children: Vec<CstNode>,
}

impl Display for CstObject {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for child in &self.children {
      write!(f, "{}", child)?;
    }
    Ok(())
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CstObjectProp {
  pub range: Range,
  pub children: Vec<CstNode>,
}

impl Display for CstObjectProp {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for child in &self.children {
      write!(f, "{}", child)?;
    }
    Ok(())
  }
}

/// Represents an object property name that may or may not be in quotes.
#[derive(Debug, PartialEq, Clone)]
pub enum ObjectPropName {
  String(CstStringLit),
  Word(CstWordLit),
}

#[derive(Debug, PartialEq, Clone)]
pub struct CstArray {
  pub range: Range,
  pub children: Vec<CstNode>,
}

impl Display for CstArray {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for child in &self.children {
      write!(f, "{}", child)?;
    }
    Ok(())
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CstToken {
  pub range: Range,
  pub char: char,
}

impl Display for CstToken {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.char)
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CstWhitespace {
  pub range: Range,
  pub text: String,
}

impl Display for CstWhitespace {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.text)
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CstComment {
  pub range: Range,
  pub text: String,
}

impl Display for CstComment {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.text)
  }
}

struct CstBuilder<'a> {
  pub text: &'a str,
  pub tokens: VecDeque<crate::tokens::TokenAndRange<'a>>,
}

impl<'a> CstBuilder<'a> {
  pub fn build(&mut self, ast_value: Option<crate::ast::Value<'a>>) -> CstRootNode {
    let mut root_node = CstRootNode { children: Vec::new() };

    if let Some(ast_value) = ast_value {
      let range = ast_value.range();
      self.scan_from_to(0, range.start, &mut root_node.children);
      root_node.children.push(self.build_value(ast_value));
      self.scan_from_to(range.end, self.text.len(), &mut root_node.children);
    } else {
      self.scan_from_to(0, self.text.len(), &mut root_node.children);
    }

    root_node
  }

  fn scan_from_to(&mut self, from: usize, to: usize, trivia: &mut Vec<CstNode>) {
    if from == to {
      return;
    }

    let mut last_from = from;
    while let Some(token) = self.tokens.front() {
      if token.range.end < from {
        self.tokens.pop_front();
      } else if token.range.start < to {
        if token.range.start > last_from {
          trivia.push(CstNode::Whitespace(CstWhitespace {
            range: Range::new(last_from, token.range.start),
            text: self.text[last_from..token.range.start].to_string(),
          }));
        }
        let token = self.tokens.pop_front().unwrap();
        match token.token {
          crate::tokens::Token::OpenBrace
          | crate::tokens::Token::CloseBrace
          | crate::tokens::Token::OpenBracket
          | crate::tokens::Token::CloseBracket
          | crate::tokens::Token::Comma
          | crate::tokens::Token::Colon => {
            trivia.push(CstNode::Token(CstToken {
              range: token.range,
              char: token.token.as_str().chars().next().unwrap(),
            }));
          }
          crate::tokens::Token::Null
          | crate::tokens::Token::String(_)
          | crate::tokens::Token::Word(_)
          | crate::tokens::Token::Boolean(_)
          | crate::tokens::Token::Number(_) => unreachable!(),
          crate::tokens::Token::CommentLine(_) | crate::tokens::Token::CommentBlock(_) => {
            trivia.push(CstNode::Comment(CstComment {
              range: token.range,
              text: self.text[token.range.start..token.range.end].to_string(),
            }));
          }
        }
        last_from = token.range.end;
      } else {
        break;
      }
    }
  }

  fn build_value(&mut self, ast_value: ast::Value<'_>) -> CstNode {
    match ast_value {
      ast::Value::StringLit(string_lit) => CstNode::StringLit(self.build_string_lit(string_lit)),
      ast::Value::NumberLit(number_lit) => CstNode::NumberLit(CstNumberLit {
        range: number_lit.range,
        value: number_lit.value.to_string(),
      }),
      ast::Value::BooleanLit(boolean_lit) => CstNode::BooleanLit(CstBooleanLit {
        range: boolean_lit.range,
        value: boolean_lit.value,
      }),
      ast::Value::Object(object) => CstNode::Object(self.build_object(object)),
      ast::Value::Array(array) => CstNode::Array(self.build_array(array)),
      ast::Value::NullKeyword(null_keyword) => CstNode::NullKeyword(CstNullKeyword {
        range: null_keyword.range,
      }),
    }
  }

  fn build_object(&mut self, object: ast::Object<'_>) -> CstObject {
    let mut last_range_end = object.range.start;
    let mut children = Vec::new();
    for prop in object.properties {
      self.scan_from_to(last_range_end, prop.range.start, &mut children);
      last_range_end = prop.range.end;
      children.push(CstNode::ObjectProp(self.build_object_prop(prop)));
    }
    self.scan_from_to(last_range_end, object.range.end, &mut children);

    CstObject {
      range: object.range,
      children,
    }
  }

  fn build_object_prop(&mut self, prop: ast::ObjectProp<'_>) -> CstObjectProp {
    let mut children = Vec::new();
    let name_range = prop.name.range();
    let value_range = prop.value.range();

    match prop.name {
      ast::ObjectPropName::String(string_lit) => {
        children.push(CstNode::StringLit(self.build_string_lit(string_lit)));
      }
      ast::ObjectPropName::Word(word_lit) => {
        children.push(CstNode::WordLit(CstWordLit {
          range: word_lit.range,
          value: word_lit.value.to_string(),
        }));
      }
    }
    self.scan_from_to(name_range.end, value_range.start, &mut children);
    children.push(self.build_value(prop.value));

    CstObjectProp {
      range: prop.range,
      children,
    }
  }

  fn build_string_lit(&mut self, string_lit: ast::StringLit<'_>) -> CstStringLit {
    CstStringLit {
      range: string_lit.range,
      value: string_lit.value.into_owned(),
    }
  }

  fn build_array(&mut self, array: ast::Array<'_>) -> CstArray {
    let mut last_range_end = array.range.start;
    let mut children = Vec::new();
    for element in array.elements {
      let element_range = element.range();
      self.scan_from_to(last_range_end, element_range.start, &mut children);
      children.push(self.build_value(element));
      last_range_end = element_range.end;
    }
    self.scan_from_to(last_range_end, array.range.end, &mut children);

    CstArray {
      range: array.range,
      children,
    }
  }
}
