use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::Display;
use std::rc::Rc;

use super::common::Ranged;
use crate::ast;
use crate::errors::ParseError;
use crate::parse_to_ast;
use crate::ParseOptions;

macro_rules! create_inner {
  ($value:expr) => {
    Rc::new(RefCell::new(CstValueInner {
      parent: None,
      value: $value,
    }))
  };
}

macro_rules! add_parent_info_methods {
  () => {
    pub fn parent(&self) -> Option<CstContainerNode> {
      self.parent_info().map(|p| p.parent)
    }

    pub fn child_index(&self) -> usize {
      self.parent_info().map(|p| p.child_index).unwrap_or(0)
    }

    pub fn previous_sibling(&self) -> Option<CstNode> {
      let parent_info = self.parent_info()?;
      if parent_info.child_index == 0 {
        return None;
      }
      parent_info.parent.child_at_index(parent_info.child_index - 1)
    }

    pub fn next_sibling(&self) -> Option<CstNode> {
      let parent_info = self.parent_info()?;
      parent_info.parent.child_at_index(parent_info.child_index + 1)
    }

    pub fn root_node(&self) -> Option<CstRootNode> {
      let mut current_node: CstNode = self.clone().into();
      while let Some(parent) = current_node.parent() {
        match parent {
          CstContainerNode::Root(node) => return Some(node),
          _ => {
            current_node = CstNode::Container(parent);
          }
        }
      }
      None
    }
  };
}

macro_rules! add_parent_methods {
  () => {
    add_parent_info_methods!();

    fn parent_info(&self) -> Option<ParentInfo> {
      self.0.borrow().parent.clone()
    }

    fn set_parent(&self, parent: Option<ParentInfo>) {
      self.0.borrow_mut().parent = parent;
    }
  };
}

macro_rules! impl_from_leaf_or_container {
  ($node_name:ident, $variant:ident, $leaf_or_container:ident, $leaf_or_container_variant:ident) => {
    impl From<$node_name> for CstNode {
      fn from(value: $node_name) -> Self {
        CstNode::$leaf_or_container_variant($leaf_or_container::$variant(value))
      }
    }

    impl From<$node_name> for $leaf_or_container {
      fn from(value: $node_name) -> Self {
        $leaf_or_container::$variant(value)
      }
    }
  };
}

macro_rules! impl_container_methods {
  ($node_name:ident, $variant:ident) => {
    impl_from_leaf_or_container!($node_name, $variant, CstContainerNode, Container);

    impl $node_name {
      add_parent_methods!();

      pub fn children(&self) -> Vec<CstNode> {
        self.0.borrow().value.clone()
      }

      pub fn children_exclude_trivia(&self) -> Vec<CstNode> {
        self
          .0
          .borrow()
          .value
          .iter()
          .filter(|n| !n.is_trivia())
          .cloned()
          .collect()
      }

      pub fn child_at_index(&self, index: usize) -> Option<CstNode> {
        self.0.borrow().value.get(index).cloned()
      }

      // destroying doesn't update the parent so this is not public
      fn destroy(&self) {
        self.clear_children();
      }

      pub fn clear_children(&self) {
        let children = std::mem::take(&mut self.0.borrow_mut().value);
        for child in children {
          child.set_parent(None);
          child.destroy();
        }
      }
    }
  };
}

macro_rules! impl_leaf_methods {
  ($node_name:ident, $variant:ident) => {
    impl_from_leaf_or_container!($node_name, $variant, CstLeafNode, Leaf);

    impl $node_name {
      add_parent_methods!();
    }
  };
}

#[derive(Clone, Debug)]
struct ParentInfo {
  pub parent: CstContainerNode,
  pub child_index: usize,
}

#[derive(Debug)]
struct CstValueInner<T> {
  parent: Option<ParentInfo>,
  value: T,
}

type CstChildrenInner = CstValueInner<Vec<CstNode>>;

#[derive(Debug, Clone)]
pub enum CstNode {
  Container(CstContainerNode),
  Leaf(CstLeafNode),
}

impl CstNode {
  add_parent_info_methods!();

  /// Gets if this node is comments, whitespace, or a non-literal token (ex. brace, colon).
  pub fn is_trivia(&self) -> bool {
    match self {
      CstNode::Leaf(leaf) => match leaf {
        CstLeafNode::BooleanLit(_)
        | CstLeafNode::NullKeyword(_)
        | CstLeafNode::NumberLit(_)
        | CstLeafNode::StringLit(_)
        | CstLeafNode::WordLit(_) => false,
        CstLeafNode::Token(_) | CstLeafNode::Whitespace(_) | CstLeafNode::Comment(_) => true,
      },
      CstNode::Container(_) => false,
    }
  }

  fn destroy(self) {
    match self {
      CstNode::Container(node) => node.destroy(),
      CstNode::Leaf(node) => node.set_parent(None),
    }
  }

  pub fn children(&self) -> Vec<CstNode> {
    match self {
      CstNode::Container(n) => n.children(),
      CstNode::Leaf(_) => Vec::new(),
    }
  }

  pub fn children_exclude_trivia(&self) -> Vec<CstNode> {
    match self {
      CstNode::Container(n) => n.children_exclude_trivia(),
      CstNode::Leaf(_) => Vec::new(),
    }
  }

  pub fn child_at_index(&self, index: usize) -> Option<CstNode> {
    match self {
      CstNode::Container(n) => n.child_at_index(index),
      CstNode::Leaf(_) => None,
    }
  }

  fn parent_info(&self) -> Option<ParentInfo> {
    match self {
      CstNode::Container(node) => node.parent_info(),
      CstNode::Leaf(node) => node.parent_info(),
    }
  }

  fn set_parent(&self, parent: Option<ParentInfo>) {
    match self {
      CstNode::Container(node) => node.set_parent(parent),
      CstNode::Leaf(node) => node.set_parent(parent),
    }
  }
}

impl Display for CstNode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      CstNode::Container(node) => node.fmt(f),
      CstNode::Leaf(node) => node.fmt(f),
    }
  }
}

// impl Ranged for CstNode {
//   fn range(&self) -> Range {
//     match self {
//       CstNode::StringLit(node) => node.range,
//       CstNode::NumberLit(node) => node.range,
//       CstNode::BooleanLit(node) => node.range,
//       CstNode::Object(node) => node.range,
//       CstNode::ObjectProp(node) => node.range,
//       CstNode::Array(node) => node.range,
//       CstNode::NullKeyword(node) => node.range,
//       CstNode::WordLit(node) => node.range,
//       CstNode::Token(node) => node.range,
//       CstNode::Whitespace(node) => node.range,
//       CstNode::Comment(node) => node.range,
//     }
//   }
// }

#[derive(Debug, Clone)]
pub enum CstContainerNode {
  Root(CstRootNode),
  Object(CstObject),
  ObjectProp(CstObjectProp),
  Array(CstArray),
}

impl CstContainerNode {
  add_parent_info_methods!();

  pub fn children(&self) -> Vec<CstNode> {
    match self {
      CstContainerNode::Root(n) => n.children(),
      CstContainerNode::Object(n) => n.children(),
      CstContainerNode::ObjectProp(n) => n.children(),
      CstContainerNode::Array(n) => n.children(),
    }
  }

  pub fn children_exclude_trivia(&self) -> Vec<CstNode> {
    match self {
      CstContainerNode::Root(n) => n.children_exclude_trivia(),
      CstContainerNode::Object(n) => n.children_exclude_trivia(),
      CstContainerNode::ObjectProp(n) => n.children_exclude_trivia(),
      CstContainerNode::Array(n) => n.children_exclude_trivia(),
    }
  }

  pub fn child_at_index(&self, index: usize) -> Option<CstNode> {
    match self {
      CstContainerNode::Root(node) => node.child_at_index(index),
      CstContainerNode::Object(node) => node.child_at_index(index),
      CstContainerNode::ObjectProp(node) => node.child_at_index(index),
      CstContainerNode::Array(node) => node.child_at_index(index),
    }
  }

  pub fn clear_children(&self) {
    match self {
      CstContainerNode::Root(n) => n.clear_children(),
      CstContainerNode::Object(n) => n.clear_children(),
      CstContainerNode::ObjectProp(n) => n.clear_children(),
      CstContainerNode::Array(n) => n.clear_children(),
    }
  }

  fn destroy(self) {
    match self {
      CstContainerNode::Root(n) => n.destroy(),
      CstContainerNode::Object(n) => n.destroy(),
      CstContainerNode::ObjectProp(n) => n.destroy(),
      CstContainerNode::Array(n) => n.destroy(),
    }
  }

  fn parent_info(&self) -> Option<ParentInfo> {
    match self {
      CstContainerNode::Root(node) => node.parent_info(),
      CstContainerNode::Object(node) => node.parent_info(),
      CstContainerNode::ObjectProp(node) => node.parent_info(),
      CstContainerNode::Array(node) => node.parent_info(),
    }
  }

  fn set_parent(&self, parent: Option<ParentInfo>) {
    match self {
      CstContainerNode::Root(node) => node.set_parent(parent),
      CstContainerNode::Object(node) => node.set_parent(parent),
      CstContainerNode::ObjectProp(node) => node.set_parent(parent),
      CstContainerNode::Array(node) => node.set_parent(parent),
    }
  }
}

impl From<CstContainerNode> for CstNode {
  fn from(value: CstContainerNode) -> Self {
    CstNode::Container(value)
  }
}

impl Display for CstContainerNode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      CstContainerNode::Root(node) => node.fmt(f),
      CstContainerNode::Object(node) => node.fmt(f),
      CstContainerNode::ObjectProp(node) => node.fmt(f),
      CstContainerNode::Array(node) => node.fmt(f),
    }
  }
}

#[derive(Debug, Clone)]
pub enum CstLeafNode {
  BooleanLit(CstBooleanLit),
  NullKeyword(CstNullKeyword),
  NumberLit(CstNumberLit),
  StringLit(CstStringLit),
  WordLit(CstWordLit),
  Token(CstToken),
  Whitespace(CstWhitespace),
  Comment(CstComment),
}

impl CstLeafNode {
  add_parent_info_methods!();

  fn parent_info(&self) -> Option<ParentInfo> {
    match self {
      CstLeafNode::BooleanLit(node) => node.parent_info(),
      CstLeafNode::NullKeyword(node) => node.parent_info(),
      CstLeafNode::NumberLit(node) => node.parent_info(),
      CstLeafNode::StringLit(node) => node.parent_info(),
      CstLeafNode::WordLit(node) => node.parent_info(),
      CstLeafNode::Token(node) => node.parent_info(),
      CstLeafNode::Whitespace(node) => node.parent_info(),
      CstLeafNode::Comment(node) => node.parent_info(),
    }
  }

  fn set_parent(&self, parent: Option<ParentInfo>) {
    match self {
      CstLeafNode::BooleanLit(node) => node.set_parent(parent),
      CstLeafNode::NullKeyword(node) => node.set_parent(parent),
      CstLeafNode::NumberLit(node) => node.set_parent(parent),
      CstLeafNode::StringLit(node) => node.set_parent(parent),
      CstLeafNode::WordLit(node) => node.set_parent(parent),
      CstLeafNode::Token(node) => node.set_parent(parent),
      CstLeafNode::Whitespace(node) => node.set_parent(parent),
      CstLeafNode::Comment(node) => node.set_parent(parent),
    }
  }
}

impl Display for CstLeafNode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      CstLeafNode::BooleanLit(node) => node.fmt(f),
      CstLeafNode::NullKeyword(node) => node.fmt(f),
      CstLeafNode::NumberLit(node) => node.fmt(f),
      CstLeafNode::StringLit(node) => node.fmt(f),
      CstLeafNode::WordLit(node) => node.fmt(f),
      CstLeafNode::Token(node) => node.fmt(f),
      CstLeafNode::Whitespace(node) => node.fmt(f),
      CstLeafNode::Comment(node) => node.fmt(f),
    }
  }
}

impl From<CstLeafNode> for CstNode {
  fn from(value: CstLeafNode) -> Self {
    CstNode::Leaf(value)
  }
}

#[derive(Debug, Clone)]
pub struct CstRootNode(Rc<RefCell<CstChildrenInner>>, Option<Rc<()>>);

impl Drop for CstRootNode {
  fn drop(&mut self) {
    if let Some(user_ref_count) = &self.1 {
      let count = Rc::strong_count(user_ref_count);
      if count == 1 {
        // this is the last remaining user reference to the root, so destroy it
        self.destroy();
      }
    }
  }
}

impl_container_methods!(CstRootNode, Root);

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

    Ok(
      CstBuilder {
        text,
        tokens: parse_result.tokens.unwrap().into_iter().collect(),
      }
      .build(parse_result.value),
    )
  }

  /// Computes the single indentation text of the file.
  pub fn single_indent_text(&self) -> Option<String> {
    let root_value = self.root_value()?;
    let first_non_trivia_child = root_value.children_exclude_trivia().get(0)?.clone();
    let mut looking_node = first_non_trivia_child;
    while let Some(previous_trivia) = looking_node.previous_sibling() {
      if let CstNode::Leaf(CstLeafNode::Whitespace(whitespace)) = &previous_trivia {
        let whitespace = whitespace.0.borrow();
        if whitespace.value.contains('\n') {
          let last_line = whitespace.value.lines().last()?;
          if !last_line.is_empty() {
            return Some(last_line.to_string());
          }
        }
      }

      looking_node = previous_trivia;
    }
    None
  }

  pub fn root_value(&self) -> Option<CstNode> {
    for child in &self.0.borrow().value {
      if !child.is_trivia() {
        return Some(child.clone());
      }
    }
    None
  }
}

impl Display for CstRootNode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for child in &self.0.borrow().value {
      write!(f, "{}", child)?;
    }
    Ok(())
  }
}

/// Node surrounded in double quotes (ex. `"my string"`).
#[derive(Debug, Clone)]
pub struct CstStringLit(Rc<RefCell<CstValueInner<String>>>);

impl_leaf_methods!(CstStringLit, StringLit);

impl CstStringLit {
  /// Sets the raw escaped value of the string.
  ///
  /// Note: This value should escape quotes otherwise a syntax error will
  /// happen when printing.
  pub fn set_raw_escaped_value(&self, value: String) {
    self.0.borrow_mut().value = value;
  }
}

impl Display for CstStringLit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "\"{}\"", self.0.borrow().value)
  }
}

#[derive(Debug, Clone)]
pub struct CstWordLit(Rc<RefCell<CstValueInner<String>>>);

impl_leaf_methods!(CstWordLit, WordLit);

impl Display for CstWordLit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0.borrow().value)
  }
}

#[derive(Debug, Clone)]
pub struct CstNumberLit(Rc<RefCell<CstValueInner<String>>>);

impl_leaf_methods!(CstNumberLit, NumberLit);

impl Display for CstNumberLit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0.borrow().value)
  }
}

/// Represents a boolean (ex. `true` or `false`).
#[derive(Debug, Clone)]
pub struct CstBooleanLit(Rc<RefCell<CstValueInner<bool>>>);

impl_leaf_methods!(CstBooleanLit, BooleanLit);

impl Display for CstBooleanLit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.0.borrow().value {
      write!(f, "true")
    } else {
      write!(f, "false")
    }
  }
}

/// Represents the null keyword (ex. `null`).
#[derive(Debug, Clone)]
pub struct CstNullKeyword(Rc<RefCell<CstValueInner<()>>>);

impl_leaf_methods!(CstNullKeyword, NullKeyword);

impl Display for CstNullKeyword {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "null")
  }
}

/// Represents an object that may contain properties (ex. `{}`, `{ "prop": 4 }`).
#[derive(Debug, Clone)]
pub struct CstObject(Rc<RefCell<CstChildrenInner>>);

impl_container_methods!(CstObject, Object);

impl Display for CstObject {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for child in &self.0.borrow().value {
      write!(f, "{}", child)?;
    }
    Ok(())
  }
}

#[derive(Debug, Clone)]
pub struct CstObjectProp(Rc<RefCell<CstChildrenInner>>);

impl_container_methods!(CstObjectProp, ObjectProp);

impl CstObjectProp {
  pub fn name(&self) -> Result<ObjectPropName, ParseError> {
    for child in &self.0.borrow().value {
      match child {
        CstNode::Leaf(CstLeafNode::StringLit(node)) => return Ok(ObjectPropName::String(node.clone())),
        CstNode::Leaf(CstLeafNode::WordLit(node)) => return Ok(ObjectPropName::Word(node.clone())),
        _ => {
          // someone may have manipulated this object such that this is no longer there
        }
      }
    }
    // todo(THIS PR): make this return an error when not found
    unreachable!();
  }

  pub fn value(&self) -> Result<CstNode, ParseError> {
    let name = self.name()?;
    let parent_info = name.parent_info().unwrap(); // todo(THIS PR): do not unwrap
    let children = &self.0.borrow().value;
    let mut children = (&children[parent_info.child_index + 1..]).iter();

    // first, skip over the colon token
    while let Some(child) = children.next() {
      if let CstNode::Leaf(CstLeafNode::Token(token)) = child {
        if token.value() == ':' {
          break;
        }
      }
    }

    // now find the value
    while let Some(child) = children.next() {
      match child {
        CstNode::Leaf(leaf) => match leaf {
          CstLeafNode::BooleanLit(_)
          | CstLeafNode::NullKeyword(_)
          | CstLeafNode::NumberLit(_)
          | CstLeafNode::StringLit(_)
          | CstLeafNode::WordLit(_) => return Ok(child.clone()),
          CstLeafNode::Token(_) | CstLeafNode::Whitespace(_) | CstLeafNode::Comment(_) => {
            // ignore
          }
        },
        CstNode::Container(container) => match container {
          CstContainerNode::Object(_) | CstContainerNode::Array(_) => return Ok(child.clone()),
          CstContainerNode::Root(_) | CstContainerNode::ObjectProp(_) => todo!(), // todo(THIS PR): surface error
        },
      }
    }

    // todo(THIS PR): make this return an error when not found
    unreachable!();
  }
}

impl Display for CstObjectProp {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for child in &self.0.borrow().value {
      write!(f, "{}", child)?;
    }
    Ok(())
  }
}

/// Represents an object property name that may or may not be in quotes.
#[derive(Debug, Clone)]
pub enum ObjectPropName {
  String(CstStringLit),
  Word(CstWordLit),
}

impl ObjectPropName {
  add_parent_info_methods!();

  fn parent_info(&self) -> Option<ParentInfo> {
    match self {
      ObjectPropName::String(n) => n.parent_info(),
      ObjectPropName::Word(n) => n.parent_info(),
    }
  }
}

impl From<ObjectPropName> for CstNode {
  fn from(value: ObjectPropName) -> Self {
    match value {
      ObjectPropName::String(n) => n.into(),
      ObjectPropName::Word(n) => n.into(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct CstArray(Rc<RefCell<CstChildrenInner>>);

impl_container_methods!(CstArray, Array);

impl CstArray {}

impl Display for CstArray {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for child in &self.0.borrow().value {
      write!(f, "{}", child)?;
    }
    Ok(())
  }
}

#[derive(Debug, Clone)]
pub struct CstToken(Rc<RefCell<CstValueInner<char>>>);

impl_leaf_methods!(CstToken, Token);

impl CstToken {
  pub fn value(&self) -> char {
    self.0.borrow().value
  }
}

impl Display for CstToken {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0.borrow().value)
  }
}

#[derive(Debug, Clone)]
pub struct CstWhitespace(Rc<RefCell<CstValueInner<String>>>);

impl_leaf_methods!(CstWhitespace, Whitespace);

impl Display for CstWhitespace {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0.borrow().value)
  }
}

#[derive(Debug, Clone)]
pub struct CstComment(Rc<RefCell<CstValueInner<String>>>);

impl_leaf_methods!(CstComment, Comment);

impl Display for CstComment {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0.borrow().value)
  }
}

struct CstBuilder<'a> {
  pub text: &'a str,
  pub tokens: VecDeque<crate::tokens::TokenAndRange<'a>>,
}

impl<'a> CstBuilder<'a> {
  pub fn build(&mut self, ast_value: Option<crate::ast::Value<'a>>) -> CstRootNode {
    let root_node = CstContainerNode::Root(CstRootNode(
      Rc::new(RefCell::new(CstChildrenInner {
        parent: None,
        value: Vec::new(),
      })),
      // ensure child nodes only get a None reference here
      None,
    ));

    if let Some(ast_value) = ast_value {
      let range = ast_value.range();
      self.scan_from_to(&root_node, 0, range.start);
      self.build_value(&root_node, ast_value);
      self.scan_from_to(&root_node, range.end, self.text.len());
    } else {
      self.scan_from_to(&root_node, 0, self.text.len());
    }

    match root_node {
      CstContainerNode::Root(node) => node,
      _ => unreachable!(),
    }
  }

  fn scan_from_to(&mut self, container: &CstContainerNode, from: usize, to: usize) {
    if from == to {
      return;
    }

    let mut last_from = from;
    while let Some(token) = self.tokens.front() {
      if token.range.end <= from {
        self.tokens.pop_front();
      } else if token.range.start < to {
        if token.range.start > last_from {
          self.build_whitespace(container, self.text[last_from..token.range.start].to_string());
        }
        let token = self.tokens.pop_front().unwrap();
        match token.token {
          crate::tokens::Token::OpenBrace
          | crate::tokens::Token::CloseBrace
          | crate::tokens::Token::OpenBracket
          | crate::tokens::Token::CloseBracket
          | crate::tokens::Token::Comma
          | crate::tokens::Token::Colon => {
            self.build_token(container, token.token.as_str().chars().next().unwrap());
          }
          crate::tokens::Token::Null
          | crate::tokens::Token::String(_)
          | crate::tokens::Token::Word(_)
          | crate::tokens::Token::Boolean(_)
          | crate::tokens::Token::Number(_) => unreachable!(
            "programming error parsing cst {:?} scanning {} to {}",
            token.token, from, to
          ),
          crate::tokens::Token::CommentLine(_) | crate::tokens::Token::CommentBlock(_) => {
            self.raw_append_child(
              container,
              CstComment(create_inner!(self.text[token.range.start..token.range.end].to_string())).into(),
            );
          }
        }
        last_from = token.range.end;
      } else {
        break;
      }
    }

    if last_from < to {
      self.build_whitespace(container, self.text[last_from..to].to_string());
    }
  }

  fn build_value(&mut self, container: &CstContainerNode, ast_value: ast::Value<'_>) {
    match ast_value {
      ast::Value::StringLit(string_lit) => self.build_string_lit(container, string_lit),
      ast::Value::NumberLit(number_lit) => self.raw_append_child(
        container,
        CstNumberLit(create_inner!(number_lit.value.to_string())).into(),
      ),
      ast::Value::BooleanLit(boolean_lit) => {
        self.raw_append_child(container, CstBooleanLit(create_inner!(boolean_lit.value)).into())
      }
      ast::Value::Object(object) => {
        let object = self.build_object(object);
        self.raw_append_child(container, object.into())
      }
      ast::Value::Array(array) => {
        let array = self.build_array(array);
        self.raw_append_child(container, array.into())
      }
      ast::Value::NullKeyword(_) => self.raw_append_child(container, CstNullKeyword(create_inner!(())).into()),
    }
  }

  fn build_object(&mut self, object: ast::Object<'_>) -> CstContainerNode {
    let container = CstContainerNode::Object(CstObject(create_inner!(Vec::new())));
    let mut last_range_end = object.range.start;
    for prop in object.properties {
      self.scan_from_to(&container, last_range_end, prop.range.start);
      last_range_end = prop.range.end;
      let object_prop = self.build_object_prop(prop);
      self.raw_append_child(&container, CstNode::Container(object_prop));
    }
    self.scan_from_to(&container, last_range_end, object.range.end);

    container
  }

  fn build_object_prop(&mut self, prop: ast::ObjectProp<'_>) -> CstContainerNode {
    let container = CstContainerNode::ObjectProp(CstObjectProp(create_inner!(Vec::new())));
    let name_range = prop.name.range();
    let value_range = prop.value.range();

    match prop.name {
      ast::ObjectPropName::String(string_lit) => {
        self.build_string_lit(&container, string_lit);
      }
      ast::ObjectPropName::Word(word_lit) => {
        self.raw_append_child(&container, CstWordLit(create_inner!(word_lit.value.to_string())).into());
      }
    }
    self.scan_from_to(&container, name_range.end, value_range.start);
    self.build_value(&container, prop.value);

    container
  }

  fn build_token(&self, container: &CstContainerNode, value: char) {
    self.raw_append_child(container, CstToken(create_inner!(value)).into());
  }

  fn build_whitespace(&self, container: &CstContainerNode, value: String) {
    if value.is_empty() {
      return;
    }
    self.raw_append_child(container, CstWhitespace(create_inner!(value)).into());
  }

  fn build_string_lit(&self, container: &CstContainerNode, lit: ast::StringLit<'_>) {
    self.raw_append_child(
      container,
      CstStringLit(create_inner!(
        self.text[lit.range.start + 1..lit.range.end - 1].to_string()
      ))
      .into(),
    );
  }

  fn build_array(&mut self, array: ast::Array<'_>) -> CstContainerNode {
    let container = CstContainerNode::Array(CstArray(create_inner!(Vec::new())));
    let mut last_range_end = array.range.start;
    for element in array.elements {
      let element_range = element.range();
      self.scan_from_to(&container, last_range_end, element_range.start);
      self.build_value(&container, element);
      last_range_end = element_range.end;
    }
    self.scan_from_to(&container, last_range_end, array.range.end);

    container
  }

  fn raw_append_child(&self, container: &CstContainerNode, child: CstNode) {
    let cloned_self = container.clone();
    let mut container = match container {
      CstContainerNode::Root(node) => node.0.borrow_mut(),
      CstContainerNode::Object(node) => node.0.borrow_mut(),
      CstContainerNode::ObjectProp(node) => node.0.borrow_mut(),
      CstContainerNode::Array(node) => node.0.borrow_mut(),
    };
    let parent_info = ParentInfo {
      parent: cloned_self,
      child_index: container.value.len(),
    };
    child.set_parent(Some(parent_info));
    container.value.push(child);
  }
}

#[cfg(test)]
mod test {
  use super::CstRootNode;

  #[test]
  fn single_indent_text() {
    let cases = [
      (
        "  ",
        r#"
{
  "prop": {
    "nested": 4
  }
}
    "#,
      ),
      (
        "  ",
        r#"
{
  /* test */ "prop": {}
}
    "#,
      ),
      (
        "    ",
        r#"
{
    /* test */  "prop": {}
}
    "#,
      ),
      (
        "\t",
        "
{
\t/* test */  \"prop\": {}
}
    ",
      ),
    ];
    for (expected, text) in cases {
      let root = build_cts(text);
      assert_eq!(root.single_indent_text(), Some(expected.to_string()));
    }
  }

  fn build_cts(text: &str) -> CstRootNode {
    CstRootNode::parse(text, &crate::ParseOptions::default()).unwrap()
  }
}
