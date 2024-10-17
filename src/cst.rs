use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::Display;
use std::rc::Rc;
use std::rc::Weak;

use super::common::Ranged;
use crate::ast;
use crate::errors::ParseError;
use crate::parse_to_ast;
use crate::string::ParseStringErrorKind;
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
      self.parent_info().map(|p| p.parent.as_container_node())
    }

    pub fn ancestors(&self) -> impl Iterator<Item = CstContainerNode> {
      AncestorIterator::new(self.clone().into())
    }

    pub fn child_index(&self) -> usize {
      self.parent_info().map(|p| p.child_index).unwrap_or(0)
    }

    pub fn previous_sibling(&self) -> Option<CstNode> {
      let parent_info = self.parent_info()?;
      if parent_info.child_index == 0 {
        return None;
      }
      parent_info
        .parent
        .as_container_node()
        .child_at_index(parent_info.child_index - 1)
    }

    pub fn previous_siblings(&self) -> impl Iterator<Item = CstNode> {
      PreviousSiblingIterator::new(self.clone().into())
    }

    pub fn next_sibling(&self) -> Option<CstNode> {
      let parent_info = self.parent_info()?;
      parent_info
        .parent
        .as_container_node()
        .child_at_index(parent_info.child_index + 1)
    }

    pub fn next_siblings(&self) -> impl Iterator<Item = CstNode> {
      NextSiblingIterator::new(self.clone().into())
    }

    pub fn root_node(&self) -> Option<CstRootNode> {
      for parent in self.ancestors() {
        if let CstContainerNode::Root(node) = parent {
          return Some(node);
        }
      }
      None
    }

    /// Returns the indentation text if it can be determined.
    pub fn indent_text(&self) -> Option<String> {
      let mut last_whitespace: Option<CstWhitespace> = None;
      for previous_sibling in self.previous_siblings() {
        match previous_sibling {
          CstNode::Container(_) => return None,
          CstNode::Leaf(leaf) => match leaf {
            CstLeafNode::Newline(_) => {
              return last_whitespace.map(|whitespace| whitespace.value());
            }
            CstLeafNode::Whitespace(whitespace) => {
              last_whitespace = Some(whitespace);
            }
            CstLeafNode::Comment(_) => {
              last_whitespace = None;
            }
            _ => return None,
          },
        }
      }
      None
    }

    /// Removes the node from the tree without making adjustments to any siblings.
    fn remove_raw(self) {
      let Some(parent_info) = self.parent_info() else {
        return; // already removed
      };
      parent_info
        .parent
        .as_container_node()
        .remove_child_set_no_parent(parent_info.child_index);
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

      pub fn children_exclude_trivia_and_tokens(&self) -> Vec<CstNode> {
        self
          .0
          .borrow()
          .value
          .iter()
          .filter(|n| !n.is_trivia() && !n.is_token())
          .cloned()
          .collect()
      }

      pub fn child_at_index(&self, index: usize) -> Option<CstNode> {
        self.0.borrow().value.get(index).cloned()
      }

      pub fn clear_children(&self) {
        let children = std::mem::take(&mut self.0.borrow_mut().value);
        for child in children {
          child.set_parent(None);
        }
      }

      fn remove_child_set_no_parent(&self, index: usize) {
        let mut inner = self.0.borrow_mut();
        if index < inner.value.len() {
          let container = self.clone().into();
          let child = inner.value.remove(index);
          child.set_parent(None);

          // update the index of the remaining children
          for index in index..inner.value.len() {
            inner.value[index].set_parent(Some(ParentInfo {
              parent: WeakParent::from_container(&container),
              child_index: index,
            }));
          }
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

      pub fn remove(self) {
        self.remove_raw()
      }
    }
  };
}

#[derive(Debug, Clone)]
enum WeakParent {
  Root(Weak<CstRootNodeInner>),
  Object(Weak<CstObjectInner>),
  ObjectProp(Weak<CstObjectPropInner>),
  Array(Weak<CstArrayInner>),
}

impl WeakParent {
  pub fn from_container(container: &CstContainerNode) -> Self {
    match container {
      CstContainerNode::Root(node) => WeakParent::Root(Rc::downgrade(&node.0)),
      CstContainerNode::Object(node) => WeakParent::Object(Rc::downgrade(&node.0)),
      CstContainerNode::ObjectProp(node) => WeakParent::ObjectProp(Rc::downgrade(&node.0)),
      CstContainerNode::Array(node) => WeakParent::Array(Rc::downgrade(&node.0)),
    }
  }

  pub fn as_container_node(&self) -> CstContainerNode {
    // It's much better to panic here to let the developer know an ancestor has been
    // lost due to being dropped because if we did something like returning None then
    // it might create strange bugs that are hard to track down.
    const PANIC_MSG: &str = "Programming error. Ensure you keep around the RootNode for the duration of using the CST.";
    match self {
      WeakParent::Root(weak) => CstRootNode(weak.upgrade().expect(PANIC_MSG)).into(),
      WeakParent::Object(weak) => CstObject(weak.upgrade().expect(PANIC_MSG)).into(),
      WeakParent::ObjectProp(weak) => CstObjectProp(weak.upgrade().expect(PANIC_MSG)).into(),
      WeakParent::Array(weak) => CstArray(weak.upgrade().expect(PANIC_MSG)).into(),
    }
  }
}

#[derive(Clone, Debug)]
struct ParentInfo {
  pub parent: WeakParent,
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

  /// Gets if this node is comments, whitespace, newlines, or a non-literal token (ex. brace, colon).
  pub fn is_trivia(&self) -> bool {
    match self {
      CstNode::Leaf(leaf) => match leaf {
        CstLeafNode::BooleanLit(_)
        | CstLeafNode::NullKeyword(_)
        | CstLeafNode::NumberLit(_)
        | CstLeafNode::StringLit(_)
        | CstLeafNode::Token(_)
        | CstLeafNode::WordLit(_) => false,
        CstLeafNode::Whitespace(_) | CstLeafNode::Newline(_) | CstLeafNode::Comment(_) => true,
      },
      CstNode::Container(_) => false,
    }
  }

  pub fn is_newline(&self) -> bool {
    match self {
      CstNode::Leaf(CstLeafNode::Newline(_)) => true,
      _ => false,
    }
  }

  pub fn is_comma(&self) -> bool {
    match self {
      CstNode::Leaf(CstLeafNode::Token(t)) => t.value() == ',',
      _ => false,
    }
  }

  pub fn is_token(&self) -> bool {
    match self {
      CstNode::Leaf(CstLeafNode::Token(_)) => true,
      _ => false,
    }
  }

  pub fn children(&self) -> Vec<CstNode> {
    match self {
      CstNode::Container(n) => n.children(),
      CstNode::Leaf(_) => Vec::new(),
    }
  }

  pub fn children_exclude_trivia_and_tokens(&self) -> Vec<CstNode> {
    match self {
      CstNode::Container(n) => n.children_exclude_trivia_and_tokens(),
      CstNode::Leaf(_) => Vec::new(),
    }
  }

  pub fn child_at_index(&self, index: usize) -> Option<CstNode> {
    match self {
      CstNode::Container(n) => n.child_at_index(index),
      CstNode::Leaf(_) => None,
    }
  }

  pub fn as_root_node(&self) -> Option<&CstRootNode> {
    match self {
      CstNode::Container(CstContainerNode::Root(node)) => Some(node),
      _ => None,
    }
  }

  pub fn as_object(&self) -> Option<&CstObject> {
    match self {
      CstNode::Container(CstContainerNode::Object(node)) => Some(node),
      _ => None,
    }
  }

  pub fn as_array(&self) -> Option<&CstArray> {
    match self {
      CstNode::Container(CstContainerNode::Array(node)) => Some(node),
      _ => None,
    }
  }

  pub fn as_object_prop(&self) -> Option<&CstObjectProp> {
    match self {
      CstNode::Container(CstContainerNode::ObjectProp(node)) => Some(node),
      _ => None,
    }
  }

  pub fn as_boolean_lit(&self) -> Option<&CstBooleanLit> {
    match self {
      CstNode::Leaf(CstLeafNode::BooleanLit(node)) => Some(node),
      _ => None,
    }
  }

  pub fn as_null_keyword(&self) -> Option<&CstNullKeyword> {
    match self {
      CstNode::Leaf(CstLeafNode::NullKeyword(node)) => Some(node),
      _ => None,
    }
  }

  pub fn as_number_lit(&self) -> Option<&CstNumberLit> {
    match self {
      CstNode::Leaf(CstLeafNode::NumberLit(node)) => Some(node),
      _ => None,
    }
  }

  pub fn as_string_lit(&self) -> Option<&CstStringLit> {
    match self {
      CstNode::Leaf(CstLeafNode::StringLit(node)) => Some(node),
      _ => None,
    }
  }

  pub fn as_word_lit(&self) -> Option<&CstWordLit> {
    match self {
      CstNode::Leaf(CstLeafNode::WordLit(node)) => Some(node),
      _ => None,
    }
  }

  pub fn as_token(&self) -> Option<&CstToken> {
    match self {
      CstNode::Leaf(CstLeafNode::Token(node)) => Some(node),
      _ => None,
    }
  }

  pub fn as_newline(&self) -> Option<&CstNewline> {
    match self {
      CstNode::Leaf(CstLeafNode::Newline(node)) => Some(node),
      _ => None,
    }
  }

  pub fn as_whitespace(&self) -> Option<&CstWhitespace> {
    match self {
      CstNode::Leaf(CstLeafNode::Whitespace(node)) => Some(node),
      _ => None,
    }
  }

  pub fn as_comment(&self) -> Option<&CstComment> {
    match self {
      CstNode::Leaf(CstLeafNode::Comment(node)) => Some(node),
      _ => None,
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

  pub fn children_exclude_trivia_and_tokens(&self) -> Vec<CstNode> {
    match self {
      CstContainerNode::Root(n) => n.children_exclude_trivia_and_tokens(),
      CstContainerNode::Object(n) => n.children_exclude_trivia_and_tokens(),
      CstContainerNode::ObjectProp(n) => n.children_exclude_trivia_and_tokens(),
      CstContainerNode::Array(n) => n.children_exclude_trivia_and_tokens(),
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

  pub fn remove_child_set_no_parent(&self, index: usize) {
    match self {
      CstContainerNode::Root(n) => n.remove_child_set_no_parent(index),
      CstContainerNode::Object(n) => n.remove_child_set_no_parent(index),
      CstContainerNode::ObjectProp(n) => n.remove_child_set_no_parent(index),
      CstContainerNode::Array(n) => n.remove_child_set_no_parent(index),
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
  Newline(CstNewline),
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
      CstLeafNode::Newline(node) => node.parent_info(),
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
      CstLeafNode::Newline(node) => node.set_parent(parent),
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
      CstLeafNode::Newline(node) => node.fmt(f),
      CstLeafNode::Comment(node) => node.fmt(f),
    }
  }
}

impl From<CstLeafNode> for CstNode {
  fn from(value: CstLeafNode) -> Self {
    CstNode::Leaf(value)
  }
}

type CstRootNodeInner = RefCell<CstChildrenInner>;
#[derive(Debug, Clone)]
pub struct CstRootNode(Rc<CstRootNodeInner>);

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
    let first_non_trivia_child = root_value.children_exclude_trivia_and_tokens().first()?.clone();
    let mut last_whitespace = None;
    for previous_trivia in first_non_trivia_child.previous_siblings() {
      match previous_trivia {
        CstNode::Leaf(CstLeafNode::Whitespace(whitespace)) => {
          last_whitespace = Some(whitespace);
        }
        CstNode::Leaf(CstLeafNode::Newline(_)) => {
          return last_whitespace.map(|whitespace| whitespace.0.borrow().value.clone());
        }
        _ => {
          last_whitespace = None;
        }
      }
    }
    None
  }

  /// Gets the root value found in the file.
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
  /// Sets the raw value of the string INCLUDING SURROUNDING QUOTES.
  pub fn set_raw_value(&self, value: String) {
    self.0.borrow_mut().value = value;
  }

  /// Gets the raw unescaped value including quotes.
  pub fn raw_value(&self) -> String {
    self.0.borrow().value.clone()
  }

  /// Gets the decoded string value.
  pub fn decoded_value(&self) -> Result<String, ParseStringErrorKind> {
    let inner = self.0.borrow();
    crate::string::parse_string(&inner.value)
      .map(|value| value.into_owned())
      .map_err(|err| err.kind)
  }
}

impl Display for CstStringLit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0.borrow().value)
  }
}

#[derive(Debug, Clone)]
pub struct CstWordLit(Rc<RefCell<CstValueInner<String>>>);

impl_leaf_methods!(CstWordLit, WordLit);

impl CstWordLit {
  /// Sets the raw value of the word literal.
  pub fn set_raw_value(&self, value: String) {
    self.0.borrow_mut().value = value;
  }
}

impl Display for CstWordLit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0.borrow().value)
  }
}

#[derive(Debug, Clone)]
pub struct CstNumberLit(Rc<RefCell<CstValueInner<String>>>);

impl_leaf_methods!(CstNumberLit, NumberLit);

impl CstNumberLit {
  /// Sets the raw string value of the number literal.
  pub fn set_raw_value(&self, value: String) {
    self.0.borrow_mut().value = value;
  }
}

impl Display for CstNumberLit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0.borrow().value)
  }
}

/// Represents a boolean (ex. `true` or `false`).
#[derive(Debug, Clone)]
pub struct CstBooleanLit(Rc<RefCell<CstValueInner<bool>>>);

impl_leaf_methods!(CstBooleanLit, BooleanLit);

impl CstBooleanLit {
  pub fn set_value(&self, value: bool) {
    self.0.borrow_mut().value = value;
  }
}

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

type CstObjectInner = RefCell<CstChildrenInner>;

/// Represents an object that may contain properties (ex. `{}`, `{ "prop": 4 }`).
#[derive(Debug, Clone)]
pub struct CstObject(Rc<CstObjectInner>);

impl_container_methods!(CstObject, Object);

impl CstObject {
  pub fn property_by_name(&self, name: &str) -> Option<CstObjectProp> {
    for child in &self.0.borrow().value {
      if let CstNode::Container(CstContainerNode::ObjectProp(prop)) = child {
        let Ok(prop_name) = prop.name() else {
          continue;
        };
        let Ok(prop_name_str) = prop_name.decoded_value() else {
          continue;
        };
        if prop_name_str == name {
          return Some(prop.clone());
        }
      }
    }
    None
  }

  pub fn properties(&self) -> Vec<CstObjectProp> {
    self
      .0
      .borrow()
      .value
      .iter()
      .filter_map(|child| match child {
        CstNode::Container(CstContainerNode::ObjectProp(prop)) => Some(prop.clone()),
        _ => None,
      })
      .collect()
  }

  // pub fn insert_property(&self, index: usize, name: &str, value: serde_json::Value) {
  //   let properties = self.properties();
  //   let previous_prop = if index == 0 { None } else { properties.get(index - 1) };
  //   let next_prop = properties.get(index);
  // }
}

impl Display for CstObject {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for child in &self.0.borrow().value {
      write!(f, "{}", child)?;
    }
    Ok(())
  }
}

type CstObjectPropInner = RefCell<CstChildrenInner>;

#[derive(Debug, Clone)]
pub struct CstObjectProp(Rc<CstObjectPropInner>);

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
    let mut children = children[parent_info.child_index + 1..].iter();

    // first, skip over the colon token
    for child in children.by_ref() {
      if let CstNode::Leaf(CstLeafNode::Token(token)) = child {
        if token.value() == ':' {
          break;
        }
      }
    }

    // now find the value
    for child in children {
      match child {
        CstNode::Leaf(leaf) => match leaf {
          CstLeafNode::BooleanLit(_)
          | CstLeafNode::NullKeyword(_)
          | CstLeafNode::NumberLit(_)
          | CstLeafNode::StringLit(_)
          | CstLeafNode::WordLit(_) => return Ok(child.clone()),
          CstLeafNode::Token(_) | CstLeafNode::Whitespace(_) | CstLeafNode::Newline(_) | CstLeafNode::Comment(_) => {
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

  pub fn previous_property(&self) -> Option<CstObjectProp> {
    for sibling in self.previous_siblings() {
      if let CstNode::Container(CstContainerNode::ObjectProp(prop)) = sibling {
        return Some(prop);
      }
    }
    None
  }

  pub fn next_property(&self) -> Option<CstObjectProp> {
    for sibling in self.next_siblings() {
      if let CstNode::Container(CstContainerNode::ObjectProp(prop)) = sibling {
        return Some(prop);
      }
    }
    None
  }

  pub fn has_trailing_comma(&self) -> bool {
    let current_node = self.clone();
    for next_sibling in current_node.next_siblings() {
      match next_sibling {
        CstNode::Container(_) => return false,
        CstNode::Leaf(leaf) => match leaf {
          CstLeafNode::BooleanLit(_)
          | CstLeafNode::NullKeyword(_)
          | CstLeafNode::NumberLit(_)
          | CstLeafNode::StringLit(_)
          | CstLeafNode::WordLit(_) => return false,
          CstLeafNode::Token(token) => {
            if token.value() == ',' {
              return true;
            } else {
              return false;
            }
          }
          CstLeafNode::Whitespace(_) | CstLeafNode::Newline(_) | CstLeafNode::Comment(_) => {
            // skip over
          }
        },
      }
    }

    false
  }

  pub fn remove(self) {
    let has_trailing_comma = self.has_trailing_comma();
    for previous in self.previous_siblings() {
      if previous.is_trivia() && !previous.is_newline() {
        previous.remove_raw();
      } else {
        break;
      }
    }

    let mut found_newline = false;
    let mut next_siblings = self.next_siblings();

    // remove up to the trailing comma
    if has_trailing_comma {
      for next in next_siblings.by_ref() {
        let is_comma = next.is_comma();
        if next.is_newline() {
          found_newline = true;
        }
        next.remove_raw();
        if is_comma {
          break;
        }
      }
    }

    // remove up to the newline
    if !found_newline {
      for next in next_siblings {
        if next.is_trivia() {
          if next.is_newline() {
            next.remove_raw();
            break;
          }
          next.remove_raw();
        } else {
          break;
        }
      }
    }

    self.remove_raw();
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

  pub fn as_string_lit(&self) -> Option<&CstStringLit> {
    match self {
      ObjectPropName::String(n) => Some(n),
      ObjectPropName::Word(_) => None,
    }
  }

  pub fn as_word_lit(&self) -> Option<&CstWordLit> {
    match self {
      ObjectPropName::String(_) => None,
      ObjectPropName::Word(n) => Some(n),
    }
  }

  pub fn decoded_value(&self) -> Result<String, ParseStringErrorKind> {
    match self {
      ObjectPropName::String(n) => n.decoded_value(),
      ObjectPropName::Word(n) => Ok(n.0.borrow().value.clone()),
    }
  }

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

type CstArrayInner = RefCell<CstChildrenInner>;

#[derive(Debug, Clone)]
pub struct CstArray(Rc<CstArrayInner>);

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

impl CstWhitespace {
  pub fn value(&self) -> String {
    self.0.borrow().value.clone()
  }
}

impl Display for CstWhitespace {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0.borrow().value)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CstNewlineKind {
  LineFeed,
  CarriageReturnLineFeed,
}

#[derive(Debug, Clone)]
pub struct CstNewline(Rc<RefCell<CstValueInner<CstNewlineKind>>>);

impl_leaf_methods!(CstNewline, Newline);

impl Display for CstNewline {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.0.borrow().value {
      CstNewlineKind::LineFeed => write!(f, "\n"),
      CstNewlineKind::CarriageReturnLineFeed => write!(f, "\r\n"),
    }
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
    let root_node = CstContainerNode::Root(CstRootNode(Rc::new(RefCell::new(CstChildrenInner {
      parent: None,
      value: Vec::new(),
    }))));

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
          self.build_whitespace(container, &self.text[last_from..token.range.start]);
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
      self.build_whitespace(container, &self.text[last_from..to]);
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

  fn build_whitespace(&self, container: &CstContainerNode, value: &str) {
    if value.is_empty() {
      return;
    }

    let mut last_found_index = 0;
    let mut chars = value.char_indices().peekable();
    let maybe_add_previous_text = |from: usize, to: usize| {
      let text = &value[from..to];
      if !text.is_empty() {
        self.raw_append_child(container, CstWhitespace(create_inner!(text.to_string())).into());
      }
    };
    while let Some((i, c)) = chars.next() {
      if c == '\r' && chars.peek().map(|(_, c)| *c) == Some('\n') {
        maybe_add_previous_text(last_found_index, i);
        self.raw_append_child(
          container,
          CstNewline(create_inner!(CstNewlineKind::CarriageReturnLineFeed)).into(),
        );
        last_found_index = i + 2;
      } else if c == '\n' {
        maybe_add_previous_text(last_found_index, i);
        self.raw_append_child(container, CstNewline(create_inner!(CstNewlineKind::LineFeed)).into());
        last_found_index = i + 1;
      }
    }

    maybe_add_previous_text(last_found_index, value.len());
  }

  fn build_string_lit(&self, container: &CstContainerNode, lit: ast::StringLit<'_>) {
    self.raw_append_child(
      container,
      CstStringLit(create_inner!(self.text[lit.range.start..lit.range.end].to_string())).into(),
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
    let weak_parent = WeakParent::from_container(container);
    let mut container = match container {
      CstContainerNode::Root(node) => node.0.borrow_mut(),
      CstContainerNode::Object(node) => node.0.borrow_mut(),
      CstContainerNode::ObjectProp(node) => node.0.borrow_mut(),
      CstContainerNode::Array(node) => node.0.borrow_mut(),
    };
    let parent_info = ParentInfo {
      parent: weak_parent,
      child_index: container.value.len(),
    };
    child.set_parent(Some(parent_info));
    container.value.push(child);
  }
}

struct AncestorIterator {
  // pre-emptively store the next ancestor in case
  // the currently returned sibling is removed
  next: Option<CstContainerNode>,
}

impl AncestorIterator {
  pub fn new(node: CstNode) -> Self {
    Self {
      next: node.parent_info().map(|i| i.parent.as_container_node()),
    }
  }
}

impl Iterator for AncestorIterator {
  type Item = CstContainerNode;

  fn next(&mut self) -> Option<Self::Item> {
    let next = self.next.take()?;
    self.next = next.parent_info().map(|i| i.parent.as_container_node());
    Some(next)
  }
}

struct NextSiblingIterator {
  // pre-emptively store the next sibling in case
  // the currently returned sibling is removed
  next: Option<CstNode>,
}

impl NextSiblingIterator {
  pub fn new(node: CstNode) -> Self {
    Self {
      next: node.next_sibling(),
    }
  }
}

impl Iterator for NextSiblingIterator {
  type Item = CstNode;

  fn next(&mut self) -> Option<Self::Item> {
    let next_sibling = self.next.take()?;
    self.next = next_sibling.next_sibling();
    Some(next_sibling)
  }
}

struct PreviousSiblingIterator {
  // pre-emptively store the previous sibling in case
  // the currently returned sibling is removed
  previous: Option<CstNode>,
}

impl PreviousSiblingIterator {
  pub fn new(node: CstNode) -> Self {
    Self {
      previous: node.previous_sibling(),
    }
  }
}

impl Iterator for PreviousSiblingIterator {
  type Item = CstNode;

  fn next(&mut self) -> Option<Self::Item> {
    let previous_sibling = self.previous.take()?;
    self.previous = previous_sibling.previous_sibling();
    Some(previous_sibling)
  }
}

#[cfg(test)]
mod test {
  use pretty_assertions::assert_eq;

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
      assert_eq!(root.single_indent_text(), Some(expected.to_string()), "Text: {}", text);
    }
  }

  #[test]
  fn modify_values() {
    let cst = build_cts(
      r#"{
    "value": 5,
    // comment
    "value2": "hello",
    value3: true
}"#,
    );

    let root_value = cst.root_value().unwrap();
    let root_obj = root_value.as_object().unwrap();
    {
      let prop = root_obj.property_by_name("value").unwrap();
      prop
        .value()
        .unwrap()
        .as_number_lit()
        .unwrap()
        .set_raw_value("10".to_string());
      assert!(prop.has_trailing_comma());
      assert!(prop.previous_property().is_none());
      assert_eq!(
        prop.next_property().unwrap().name().unwrap().decoded_value().unwrap(),
        "value2"
      );
      assert_eq!(prop.indent_text().unwrap(), "    ");
    }
    {
      let prop = root_obj.property_by_name("value2").unwrap();
      prop
        .value()
        .unwrap()
        .as_string_lit()
        .unwrap()
        .set_raw_value("\"5\"".to_string());
      assert!(prop.has_trailing_comma());
      assert_eq!(
        prop
          .previous_property()
          .unwrap()
          .name()
          .unwrap()
          .decoded_value()
          .unwrap(),
        "value"
      );
      assert_eq!(
        prop.next_property().unwrap().name().unwrap().decoded_value().unwrap(),
        "value3"
      );
    }
    {
      let prop = root_obj.property_by_name("value3").unwrap();
      prop.value().unwrap().as_boolean_lit().unwrap().set_value(false);
      prop
        .name()
        .unwrap()
        .as_word_lit()
        .unwrap()
        .set_raw_value("value4".to_string());
      assert!(!prop.has_trailing_comma());
      assert_eq!(
        prop
          .previous_property()
          .unwrap()
          .name()
          .unwrap()
          .decoded_value()
          .unwrap(),
        "value2"
      );
      assert!(prop.next_property().is_none());
    }

    assert_eq!(
      cst.to_string(),
      r#"{
    "value": 10,
    // comment
    "value2": "5",
    value4: false
}"#
    );
  }

  #[test]
  fn remove_properties() {
    fn run_test(prop_name: &str, json: &str, expected: &str) {
      let cst = build_cts(json);
      let root_value = cst.root_value().unwrap();
      let root_obj = root_value.as_object().unwrap();
      let prop = root_obj.property_by_name(prop_name).unwrap();
      prop.remove();
      assert_eq!(cst.to_string(), expected);
    }

    run_test(
      "value2",
      r#"{
    "value": 5,
    // comment
    "value2": "hello",
    value3: true
}"#,
      r#"{
    "value": 5,
    // comment
    value3: true
}"#,
    );

    run_test(
      "value2",
      r#"{
    "value": 5,
    // comment
    "value2": "hello"
    ,value3: true
}"#,
      // this is fine... people doing stupid things
      r#"{
    "value": 5,
    // comment
value3: true
}"#,
    );
  }

  fn build_cts(text: &str) -> CstRootNode {
    CstRootNode::parse(text, &crate::ParseOptions::default()).unwrap()
  }
}
