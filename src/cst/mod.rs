use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::Display;
use std::iter::Peekable;
use std::rc::Rc;
use std::rc::Weak;

use super::common::Ranged;
use crate::ast;
use crate::errors::ParseError;
use crate::parse_to_ast;
use crate::string::ParseStringErrorKind;
use crate::ParseOptions;

mod input;

pub use input::*;

macro_rules! add_root_node_method {
  () => {
    /// Gets the root node.
    ///
    /// Returns `None` if this node has become disconnected from
    /// the tree by being removed.
    pub fn root_node(&self) -> Option<CstRootNode> {
      self
        .ancestors()
        .filter_map(|parent| match parent {
          CstContainerNode::Root(node) => Some(node),
          _ => None,
        })
        .next()
    }
  };
}

macro_rules! add_parent_info_methods {
  () => {
    /// Parent of the node.
    ///
    /// Returns `None` if this node has become disconnected from
    /// the tree by being removed.
    pub fn parent(&self) -> Option<CstContainerNode> {
      self.parent_info().map(|p| p.parent.as_container_node())
    }

    /// An iterator of ancestors of this node.
    pub fn ancestors(&self) -> impl Iterator<Item = CstContainerNode> {
      AncestorIterator::new(self.clone().into())
    }

    /// Current child index of the node within the children of the
    /// parent node.
    pub fn child_index(&self) -> usize {
      self.parent_info().map(|p| p.child_index).unwrap_or(0)
    }

    /// Node that comes before this one that shares the same parent.
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

    /// Siblings coming before this node. This does not
    /// include cousins.
    pub fn previous_siblings(&self) -> impl Iterator<Item = CstNode> {
      PreviousSiblingIterator::new(self.clone().into())
    }

    /// Node that comes after this one that shares the same parent.
    pub fn next_sibling(&self) -> Option<CstNode> {
      let parent_info = self.parent_info()?;
      parent_info
        .parent
        .as_container_node()
        .child_at_index(parent_info.child_index + 1)
    }

    /// Siblings coming after this node. This does not
    /// include cousins.
    pub fn next_siblings(&self) -> impl Iterator<Item = CstNode> {
      NextSiblingIterator::new(self.clone().into())
    }

    /// Returns the indentation text if it can be determined.
    pub fn indent_text(&self) -> Option<String> {
      indent_text(&self.clone().into())
    }

    /// Gets the trailing comma token of the node, if it exists.
    pub fn trailing_comma(&self) -> Option<CstToken> {
      find_trailing_comma(&self.clone().into())
    }

    /// Infers if the node or appropriate ancestor uses trailing commas.
    pub fn uses_trailing_commas(&self) -> bool {
      uses_trailing_commas(self.clone().into())
    }
  };
}

fn find_trailing_comma(node: &CstNode) -> Option<CstToken> {
  for next_sibling in node.next_siblings() {
    match next_sibling {
      CstNode::Container(_) => return None,
      CstNode::Leaf(leaf) => match leaf {
        CstLeafNode::BooleanLit(_)
        | CstLeafNode::NullKeyword(_)
        | CstLeafNode::NumberLit(_)
        | CstLeafNode::StringLit(_)
        | CstLeafNode::WordLit(_) => return None,
        CstLeafNode::Token(token) => {
          if token.value() == ',' {
            return Some(token);
          } else {
            return None;
          }
        }
        CstLeafNode::Whitespace(_) | CstLeafNode::Newline(_) | CstLeafNode::Comment(_) => {
          // skip over
        }
      },
    }
  }

  None
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

      /// Children of the current node.
      pub fn children(&self) -> Vec<CstNode> {
        self.0.borrow().value.clone()
      }

      /// Children of the current node excluding comments, whitespace, newlines, and tokens.
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

      /// Gets the child at the specified index.
      pub fn child_at_index(&self, index: usize) -> Option<CstNode> {
        self.0.borrow().value.get(index).cloned()
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
      add_root_node_method!();
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

impl<T> CstValueInner<T> {
  fn new(value: T) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(CstValueInner { parent: None, value }))
  }
}

type CstChildrenInner = CstValueInner<Vec<CstNode>>;

/// All the different kinds of nodes that can appear in the CST.
#[derive(Debug, Clone)]
pub enum CstNode {
  Container(CstContainerNode),
  Leaf(CstLeafNode),
}

impl CstNode {
  add_parent_info_methods!();
  add_root_node_method!();

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

  /// Comments that become before this one on the same line.
  pub fn leading_comments_same_line(&self) -> impl Iterator<Item = CstComment> {
    self
      .previous_siblings()
      .take_while(|n| n.is_whitespace() || n.is_comment())
      .filter_map(|n| match n {
        CstNode::Leaf(CstLeafNode::Comment(comment)) => Some(comment.clone()),
        _ => None,
      })
  }

  /// Comments that come after this one on the same line.
  ///
  /// Only returns owned trailing comments on the same line and not if owned by the next node.
  pub fn trailing_comments_same_line(&self) -> impl Iterator<Item = CstComment> {
    // ensure the trailing comments are owned
    for sibling in self.next_siblings() {
      if sibling.is_newline() {
        break;
      } else if !sibling.is_comment() && !sibling.is_whitespace() {
        return Box::new(std::iter::empty()) as Box<dyn Iterator<Item = CstComment>>;
      }
    }

    Box::new(
      self
        .next_siblings()
        .take_while(|n| n.is_whitespace() || n.is_comment())
        .filter_map(|n| match n {
          CstNode::Leaf(CstLeafNode::Comment(comment)) => Some(comment.clone()),
          _ => None,
        }),
    )
  }

  /// If this node is a newline.
  pub fn is_newline(&self) -> bool {
    matches!(self, CstNode::Leaf(CstLeafNode::Newline(_)))
  }

  /// If this node is a comma.
  pub fn is_comma(&self) -> bool {
    match self {
      CstNode::Leaf(CstLeafNode::Token(t)) => t.value() == ',',
      _ => false,
    }
  }

  /// If this node is a comment.
  pub fn is_comment(&self) -> bool {
    matches!(self, CstNode::Leaf(CstLeafNode::Comment(_)))
  }

  /// If this node is a token.
  pub fn is_token(&self) -> bool {
    matches!(self, CstNode::Leaf(CstLeafNode::Token(_)))
  }

  /// If this node is whitespace.
  pub fn is_whitespace(&self) -> bool {
    matches!(self, CstNode::Leaf(CstLeafNode::Whitespace(_)))
  }

  /// Token char of the node if it's a token.
  pub fn token_char(&self) -> Option<char> {
    match self {
      CstNode::Leaf(CstLeafNode::Token(token)) => Some(token.value()),
      _ => None,
    }
  }

  /// Children of this node.
  pub fn children(&self) -> Vec<CstNode> {
    match self {
      CstNode::Container(n) => n.children(),
      CstNode::Leaf(_) => Vec::new(),
    }
  }

  /// Children of the current node excluding comments, whitespace, newlines, and tokens.
  pub fn children_exclude_trivia_and_tokens(&self) -> Vec<CstNode> {
    match self {
      CstNode::Container(n) => n.children_exclude_trivia_and_tokens(),
      CstNode::Leaf(_) => Vec::new(),
    }
  }

  /// Child at the specified index.
  pub fn child_at_index(&self, index: usize) -> Option<CstNode> {
    match self {
      CstNode::Container(n) => n.child_at_index(index),
      CstNode::Leaf(_) => None,
    }
  }

  /// Node if it's the root node.
  pub fn as_root_node(&self) -> Option<&CstRootNode> {
    match self {
      CstNode::Container(CstContainerNode::Root(node)) => Some(node),
      _ => None,
    }
  }

  /// Node if it's an object.
  pub fn as_object(&self) -> Option<&CstObject> {
    match self {
      CstNode::Container(CstContainerNode::Object(node)) => Some(node),
      _ => None,
    }
  }

  /// Node if it's an array.
  pub fn as_array(&self) -> Option<&CstArray> {
    match self {
      CstNode::Container(CstContainerNode::Array(node)) => Some(node),
      _ => None,
    }
  }

  /// Node if it's an object property.
  pub fn as_object_prop(&self) -> Option<&CstObjectProp> {
    match self {
      CstNode::Container(CstContainerNode::ObjectProp(node)) => Some(node),
      _ => None,
    }
  }

  /// Node if it's a boolean literal.
  pub fn as_boolean_lit(&self) -> Option<&CstBooleanLit> {
    match self {
      CstNode::Leaf(CstLeafNode::BooleanLit(node)) => Some(node),
      _ => None,
    }
  }

  /// Node if it's a null keyword.
  pub fn as_null_keyword(&self) -> Option<&CstNullKeyword> {
    match self {
      CstNode::Leaf(CstLeafNode::NullKeyword(node)) => Some(node),
      _ => None,
    }
  }

  /// Node if it's a number literal.
  pub fn as_number_lit(&self) -> Option<&CstNumberLit> {
    match self {
      CstNode::Leaf(CstLeafNode::NumberLit(node)) => Some(node),
      _ => None,
    }
  }

  /// Node if it's a string literal.
  pub fn as_string_lit(&self) -> Option<&CstStringLit> {
    match self {
      CstNode::Leaf(CstLeafNode::StringLit(node)) => Some(node),
      _ => None,
    }
  }

  /// Node if it's a word literal.
  pub fn as_word_lit(&self) -> Option<&CstWordLit> {
    match self {
      CstNode::Leaf(CstLeafNode::WordLit(node)) => Some(node),
      _ => None,
    }
  }

  /// Node if it's a token.
  pub fn as_token(&self) -> Option<&CstToken> {
    match self {
      CstNode::Leaf(CstLeafNode::Token(node)) => Some(node),
      _ => None,
    }
  }

  /// Node if it's a newline.
  pub fn as_newline(&self) -> Option<&CstNewline> {
    match self {
      CstNode::Leaf(CstLeafNode::Newline(node)) => Some(node),
      _ => None,
    }
  }

  /// Node if it's whitespace.
  pub fn as_whitespace(&self) -> Option<&CstWhitespace> {
    match self {
      CstNode::Leaf(CstLeafNode::Whitespace(node)) => Some(node),
      _ => None,
    }
  }

  /// Node if it's a comment.
  pub fn as_comment(&self) -> Option<&CstComment> {
    match self {
      CstNode::Leaf(CstLeafNode::Comment(node)) => Some(node),
      _ => None,
    }
  }

  /// Removes the node from the JSON.
  ///
  /// Note: Removing certain nodes may cause syntax errors.
  pub fn remove(self) {
    match self {
      CstNode::Container(n) => n.remove(),
      CstNode::Leaf(n) => n.remove(),
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
}

impl Display for CstNode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      CstNode::Container(node) => node.fmt(f),
      CstNode::Leaf(node) => node.fmt(f),
    }
  }
}

#[derive(Default, Debug, Clone)]
struct StyleInfo {
  pub uses_trailing_commas: bool,
  pub newline_kind: CstNewlineKind,
}

#[derive(Debug, Clone)]
pub enum CstContainerNode {
  Root(CstRootNode),
  Array(CstArray),
  Object(CstObject),
  ObjectProp(CstObjectProp),
}

impl CstContainerNode {
  add_parent_info_methods!();
  add_root_node_method!();

  /// If this is the root node.
  pub fn is_root(&self) -> bool {
    matches!(self, CstContainerNode::Root(_))
  }

  /// If this is an array node.
  pub fn is_array(&self) -> bool {
    matches!(self, CstContainerNode::Array(_))
  }

  /// If this is an object node.
  pub fn is_object(&self) -> bool {
    matches!(self, CstContainerNode::Object(_))
  }

  /// If this is an object property node.
  pub fn is_object_prop(&self) -> bool {
    matches!(self, CstContainerNode::ObjectProp(_))
  }

  /// Node if it's the root node.
  pub fn as_root(&self) -> Option<&CstRootNode> {
    match self {
      CstContainerNode::Root(node) => Some(node),
      _ => None,
    }
  }

  /// Node if it's an array.
  pub fn as_array(&self) -> Option<&CstArray> {
    match self {
      CstContainerNode::Array(node) => Some(node),
      _ => None,
    }
  }

  /// Node if it's an object.
  pub fn as_object(&self) -> Option<&CstObject> {
    match self {
      CstContainerNode::Object(node) => Some(node),
      _ => None,
    }
  }

  /// Node if it's an object property.
  pub fn as_object_prop(&self) -> Option<&CstObjectProp> {
    match self {
      CstContainerNode::ObjectProp(node) => Some(node),
      _ => None,
    }
  }

  /// Children of the node.
  pub fn children(&self) -> Vec<CstNode> {
    match self {
      CstContainerNode::Root(n) => n.children(),
      CstContainerNode::Object(n) => n.children(),
      CstContainerNode::ObjectProp(n) => n.children(),
      CstContainerNode::Array(n) => n.children(),
    }
  }

  /// Children of the current node excluding comments, whitespace, newlines, and tokens.
  pub fn children_exclude_trivia_and_tokens(&self) -> Vec<CstNode> {
    match self {
      CstContainerNode::Root(n) => n.children_exclude_trivia_and_tokens(),
      CstContainerNode::Object(n) => n.children_exclude_trivia_and_tokens(),
      CstContainerNode::ObjectProp(n) => n.children_exclude_trivia_and_tokens(),
      CstContainerNode::Array(n) => n.children_exclude_trivia_and_tokens(),
    }
  }

  /// Child at the specified index.
  pub fn child_at_index(&self, index: usize) -> Option<CstNode> {
    match self {
      CstContainerNode::Root(node) => node.child_at_index(index),
      CstContainerNode::Object(node) => node.child_at_index(index),
      CstContainerNode::ObjectProp(node) => node.child_at_index(index),
      CstContainerNode::Array(node) => node.child_at_index(index),
    }
  }

  fn remove_child_set_no_parent(&self, index: usize) {
    match self {
      CstContainerNode::Root(n) => n.remove_child_set_no_parent(index),
      CstContainerNode::Object(n) => n.remove_child_set_no_parent(index),
      CstContainerNode::ObjectProp(n) => n.remove_child_set_no_parent(index),
      CstContainerNode::Array(n) => n.remove_child_set_no_parent(index),
    }
  }

  /// Removes the node from the JSON.
  pub fn remove(self) {
    match self {
      CstContainerNode::Root(n) => n.clear_children(),
      CstContainerNode::Object(n) => n.remove(),
      CstContainerNode::ObjectProp(n) => n.remove(),
      CstContainerNode::Array(n) => n.remove(),
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

  #[inline(always)]
  fn raw_append_child(&self, child: CstNode) {
    self.raw_insert_child(None, child);
  }

  #[inline(always)]
  fn raw_insert_child(&self, index: Option<&mut usize>, child: CstNode) {
    self.raw_insert_children(index, vec![child]);
  }

  #[inline(always)]
  fn raw_append_children(&self, children: Vec<CstNode>) {
    self.raw_insert_children(None, children);
  }

  fn raw_insert_children(&self, index: Option<&mut usize>, children: Vec<CstNode>) {
    if children.is_empty() {
      return;
    }

    let weak_parent = WeakParent::from_container(self);
    let mut container = match self {
      CstContainerNode::Root(node) => node.0.borrow_mut(),
      CstContainerNode::Object(node) => node.0.borrow_mut(),
      CstContainerNode::ObjectProp(node) => node.0.borrow_mut(),
      CstContainerNode::Array(node) => node.0.borrow_mut(),
    };
    let insert_index = index.as_ref().map(|i| **i).unwrap_or(container.value.len());
    if let Some(i) = index {
      *i += children.len();
    }
    container.value.splice(insert_index..insert_index, children);

    // update the child index of all the nodes
    for (i, child) in container.value.iter().enumerate().skip(insert_index) {
      child.set_parent(Some(ParentInfo {
        parent: weak_parent.clone(),
        child_index: i,
      }));
    }
  }

  fn raw_insert_value_with_internal_indent(
    &self,
    insert_index: Option<&mut usize>,
    value: InsertValue,
    style_info: &StyleInfo,
    indents: &Indents,
  ) {
    match value {
      InsertValue::Value(value) => {
        let is_multiline = value.force_multiline();
        match value {
          CstInputValue::Null => {
            self.raw_insert_child(insert_index, CstLeafNode::NullKeyword(CstNullKeyword::new()).into());
          }
          CstInputValue::Bool(value) => {
            self.raw_insert_child(insert_index, CstLeafNode::BooleanLit(CstBooleanLit::new(value)).into());
          }
          CstInputValue::Number(value) => {
            self.raw_insert_child(insert_index, CstLeafNode::NumberLit(CstNumberLit::new(value)).into());
          }
          CstInputValue::String(value) => {
            self.raw_insert_child(
              insert_index,
              CstLeafNode::StringLit(CstStringLit::new_escaped(&value)).into(),
            );
          }
          CstInputValue::Array(elements) => {
            let array_node: CstContainerNode = CstArray::new().into();
            self.raw_insert_child(insert_index, array_node.clone().into());

            array_node.raw_append_child(CstToken::new('[').into());
            if !elements.is_empty() {
              let indents = indents.indent();
              let mut elements = elements.into_iter().peekable();
              while let Some(value) = elements.next() {
                if is_multiline {
                  array_node.raw_insert_children(
                    None,
                    vec![
                      CstNewline::new(style_info.newline_kind).into(),
                      CstWhitespace::new(indents.current_indent.clone()).into(),
                    ],
                  );
                }

                array_node.raw_insert_value_with_internal_indent(None, InsertValue::Value(value), style_info, &indents);

                if style_info.uses_trailing_commas && is_multiline || elements.peek().is_some() {
                  if is_multiline {
                    array_node.raw_append_child(CstToken::new(',').into());
                  } else {
                    array_node.raw_insert_children(
                      None,
                      vec![CstToken::new(',').into(), CstWhitespace::new(" ".to_string()).into()],
                    );
                  }
                }
              }
            }

            if is_multiline {
              array_node.raw_append_children(vec![
                CstNewline::new(style_info.newline_kind).into(),
                CstWhitespace::new(indents.current_indent.clone()).into(),
              ]);
            }

            array_node.raw_append_child(CstToken::new(']').into());
          }
          CstInputValue::Object(properties) => {
            let object_node: CstContainerNode = CstObject::new().into();
            self.raw_insert_child(insert_index, object_node.clone().into());

            object_node.raw_append_child(CstToken::new('{').into());

            if !properties.is_empty() {
              {
                let indents = indents.indent();
                let mut properties = properties.into_iter().peekable();
                while let Some((prop_name, value)) = properties.next() {
                  object_node.raw_append_child(CstNewline::new(style_info.newline_kind).into());
                  object_node.raw_append_child(CstWhitespace::new(indents.current_indent.clone()).into());
                  object_node.raw_insert_value_with_internal_indent(
                    None,
                    InsertValue::Property(&prop_name, value),
                    style_info,
                    &indents,
                  );
                  if style_info.uses_trailing_commas || properties.peek().is_some() {
                    object_node.raw_append_child(CstToken::new(',').into());
                  }
                }
              }

              object_node.raw_append_children(vec![
                CstNewline::new(style_info.newline_kind).into(),
                CstWhitespace::new(indents.current_indent.clone()).into(),
              ]);
            }

            object_node.raw_append_child(CstToken::new('}').into());
          }
        }
      }
      InsertValue::Property(prop_name, value) => {
        let prop = CstContainerNode::ObjectProp(CstObjectProp::new());
        self.raw_insert_child(insert_index, prop.clone().into());
        prop.raw_insert_children(
          None,
          vec![
            CstStringLit::new_escaped(prop_name).into(),
            CstToken::new(':').into(),
            CstWhitespace::new(" ".to_string()).into(),
          ],
        );
        prop.raw_insert_value_with_internal_indent(None, InsertValue::Value(value), style_info, indents);
      }
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
  add_root_node_method!();

  /// Removes the node from the JSON.
  pub fn remove(self) {
    match self {
      CstLeafNode::BooleanLit(n) => n.remove(),
      CstLeafNode::NullKeyword(n) => n.remove(),
      CstLeafNode::NumberLit(n) => n.remove(),
      CstLeafNode::StringLit(n) => n.remove(),
      CstLeafNode::WordLit(n) => n.remove(),
      CstLeafNode::Token(n) => n.remove(),
      CstLeafNode::Whitespace(n) => n.remove(),
      CstLeafNode::Newline(n) => n.remove(),
      CstLeafNode::Comment(n) => n.remove(),
    }
  }

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
  /// Parses the text into a CST.
  ///
  /// WARNING: You MUST not drop the root node for the duration of using the CST
  /// or a panic could occur in certain scenarios. This is because the CST uses weak
  /// references for ancestors and if the root node is dropped then the weak reference
  /// will be lost and the CST will panic to prevent bugs when a descendant node
  /// attempts to access an ancestor that was dropped.
  ///
  /// ```
  /// use jsonc_parser::cst::CstRootNode;
  /// use jsonc_parser::ParseOptions;
  /// use jsonc_parser::value;
  ///
  /// let json_text = r#"{
  ///   "data": 123
  /// }"#;
  ///
  /// let node = CstRootNode::parse(json_text, &ParseOptions::default()).unwrap();
  /// let root_value = node.root_value().unwrap();
  /// let root_obj = root_value.as_object().unwrap();
  ///
  /// root_obj.append("new_key", value!([456, 789, false]));
  ///
  /// assert_eq!(node.to_string(), r#"{
  ///   "data": 123,
  ///   "new_key": [456, 789, false]
  /// }"#);
  /// ```
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

  /// Newline kind used within the JSON text.
  pub fn newline_kind(&self) -> CstNewlineKind {
    let mut current_children: VecDeque<CstContainerNode> = VecDeque::from([self.clone().into()]);
    while let Some(child) = current_children.pop_front() {
      for child in child.children() {
        if let CstNode::Container(child) = child {
          current_children.push_back(child);
        } else if let CstNode::Leaf(CstLeafNode::Newline(node)) = child {
          return node.kind();
        }
      }
    }
    CstNewlineKind::LineFeed
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

  /// Sets potentially replacing the root value found in the JSON document.
  pub fn set_root_value(&self, root_value: CstInputValue) {
    let container: CstContainerNode = self.clone().into();
    let style_info = StyleInfo {
      newline_kind: self.newline_kind(),
      uses_trailing_commas: uses_trailing_commas(self.clone().into()),
    };
    let indents = compute_indents(&self.clone().into());
    let mut insert_index = if let Some(root_value) = self.root_value() {
      let index = root_value.child_index();
      root_value.remove();
      index
    } else {
      let children = self.children();
      let mut index = match children.last() {
        Some(CstNode::Leaf(CstLeafNode::Newline(_))) => children.len() - 1,
        _ => children.len(),
      };
      let previous_node = if index == 0 { None } else { children.get(index - 1) };
      if let Some(CstNode::Leaf(CstLeafNode::Comment(_))) = previous_node {
        // insert a newline if the last node before is a comment
        container.raw_insert_child(Some(&mut index), CstNewline::new(style_info.newline_kind).into());
      }
      if self.child_at_index(index).is_none() {
        // insert a trailing newline
        container.raw_insert_child(Some(&mut index), CstNewline::new(style_info.newline_kind).into());
        index -= 1;
      }
      index
    };
    container.raw_insert_value_with_internal_indent(
      Some(&mut insert_index),
      InsertValue::Value(root_value),
      &style_info,
      &indents,
    );
  }

  /// Gets or creates the root value as an object, return Some if successful
  /// or None if the root value already exists and is not an object.
  pub fn ensure_object_value(&self) -> Option<CstObject> {
    match self.root_value() {
      Some(CstNode::Container(CstContainerNode::Object(node))) => Some(node),
      Some(_) => None,
      None => {
        self.set_root_value(CstInputValue::Object(Vec::new()));
        // should always work, but might as well do this
        self.root_value().and_then(|o| o.as_object().cloned())
      }
    }
  }

  /// Clears all the children from the root node making it empty.
  pub fn clear_children(&self) {
    let children = std::mem::take(&mut self.0.borrow_mut().value);
    for child in children {
      child.set_parent(None);
    }
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
  fn new(value: String) -> Self {
    Self(CstValueInner::new(value))
  }

  fn new_escaped(value: &str) -> Self {
    Self::new(format!("\"{}\"", value.replace("\"", "\\\"")))
  }

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

  /// Replaces this node with a new value.
  pub fn replace_with(self, replacement: CstInputValue) -> Option<CstNode> {
    replace_with(self.into(), InsertValue::Value(replacement))
  }

  /// Removes the node from the JSON.
  pub fn remove(self) {
    remove_comma_separated(self.into())
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
  fn new(value: String) -> Self {
    Self(CstValueInner::new(value))
  }

  /// Sets the raw value of the word literal.
  pub fn set_raw_value(&self, value: String) {
    self.0.borrow_mut().value = value;
  }

  /// Replaces this node with a new value.
  pub fn replace_with(self, replacement: CstInputValue) -> Option<CstNode> {
    replace_with(self.into(), InsertValue::Value(replacement))
  }

  /// Removes the node from the JSON.
  pub fn remove(self) {
    remove_comma_separated(self.into())
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
  fn new(value: String) -> Self {
    Self(CstValueInner::new(value))
  }

  /// Sets the raw string value of the number literal.
  pub fn set_raw_value(&self, value: String) {
    self.0.borrow_mut().value = value;
  }

  /// Replaces this node with a new value.
  pub fn replace_with(self, replacement: CstInputValue) -> Option<CstNode> {
    replace_with(self.into(), InsertValue::Value(replacement))
  }

  /// Removes the node from the JSON.
  pub fn remove(self) {
    remove_comma_separated(self.into())
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
  fn new(value: bool) -> Self {
    Self(CstValueInner::new(value))
  }

  /// Gets the value of the boolean literal.
  pub fn value(&self) -> bool {
    self.0.borrow().value
  }

  /// Sets the value of the boolean literal.
  pub fn set_value(&self, value: bool) {
    self.0.borrow_mut().value = value;
  }

  /// Replaces this node with a new value.
  pub fn replace_with(self, replacement: CstInputValue) -> Option<CstNode> {
    replace_with(self.into(), InsertValue::Value(replacement))
  }

  /// Removes the node from the JSON.
  pub fn remove(self) {
    remove_comma_separated(self.into())
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

impl CstNullKeyword {
  fn new() -> Self {
    Self(CstValueInner::new(()))
  }

  /// Replaces this node with a new value.
  pub fn replace_with(self, replacement: CstInputValue) -> Option<CstNode> {
    replace_with(self.into(), InsertValue::Value(replacement))
  }

  /// Removes the node from the JSON.
  pub fn remove(self) {
    remove_comma_separated(self.into())
  }
}

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
  add_root_node_method!();

  fn new() -> Self {
    Self(CstValueInner::new(Vec::new()))
  }

  /// Array property by name.
  ///
  /// Returns `None` if the property doesn't exist or is not an array.
  pub fn get_array(&self, name: &str) -> Option<CstArray> {
    match self.get(name)?.value()? {
      CstNode::Container(CstContainerNode::Array(node)) => Some(node),
      _ => None,
    }
  }

  /// Object property by name.
  ///
  /// Returns `None` if the property doesn't exist or is not an object.
  pub fn get_object(&self, name: &str) -> Option<CstObject> {
    match self.get(name)?.value()? {
      CstNode::Container(CstContainerNode::Object(node)) => Some(node),
      _ => None,
    }
  }

  /// Property by name.
  ///
  /// Returns `None` if the property doesn't exist.
  pub fn get(&self, name: &str) -> Option<CstObjectProp> {
    for child in &self.0.borrow().value {
      if let CstNode::Container(CstContainerNode::ObjectProp(prop)) = child {
        let Some(prop_name) = prop.name() else {
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

  /// Properties of the object.
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

  /// Appends a property to the object.
  pub fn append(&self, prop_name: &str, value: CstInputValue) {
    self.insert_or_append(None, prop_name, value);
  }

  /// Inserts a property at the specified index.
  pub fn insert(&self, index: usize, prop_name: &str, value: CstInputValue) {
    self.insert_or_append(Some(index), prop_name, value);
  }

  fn insert_or_append(&self, index: Option<usize>, prop_name: &str, value: CstInputValue) {
    insert_or_append_to_container(
      &CstContainerNode::Object(self.clone()),
      self.properties().into_iter().map(|c| c.into()).collect(),
      index,
      InsertValue::Property(prop_name, value),
    )
  }

  /// Replaces this node with a new value.
  pub fn replace_with(self, replacement: CstInputValue) -> Option<CstNode> {
    replace_with(self.into(), InsertValue::Value(replacement))
  }

  /// Removes the node from the JSON.
  pub fn remove(self) {
    remove_comma_separated(self.into())
  }
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
  add_root_node_method!();

  fn new() -> Self {
    Self(CstValueInner::new(Vec::new()))
  }

  /// Name of the object property.
  ///
  /// Returns `None` if the name doesn't exist.
  pub fn name(&self) -> Option<ObjectPropName> {
    for child in &self.0.borrow().value {
      match child {
        CstNode::Leaf(CstLeafNode::StringLit(node)) => return Some(ObjectPropName::String(node.clone())),
        CstNode::Leaf(CstLeafNode::WordLit(node)) => return Some(ObjectPropName::Word(node.clone())),
        _ => {
          // someone may have manipulated this object such that this is no longer there
        }
      }
    }
    None
  }

  /// Value of the object property.
  ///
  /// Returns `None` if the value doesn't exist.
  pub fn value(&self) -> Option<CstNode> {
    let name = self.name()?;
    let parent_info = name.parent_info()?;
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
          | CstLeafNode::WordLit(_) => return Some(child.clone()),
          CstLeafNode::Token(_) | CstLeafNode::Whitespace(_) | CstLeafNode::Newline(_) | CstLeafNode::Comment(_) => {
            // ignore
          }
        },
        CstNode::Container(container) => match container {
          CstContainerNode::Object(_) | CstContainerNode::Array(_) => return Some(child.clone()),
          CstContainerNode::Root(_) | CstContainerNode::ObjectProp(_) => return None,
        },
      }
    }

    None
  }

  /// Sibling object property coming before this one.
  pub fn previous_property(&self) -> Option<CstObjectProp> {
    for sibling in self.previous_siblings() {
      if let CstNode::Container(CstContainerNode::ObjectProp(prop)) = sibling {
        return Some(prop);
      }
    }
    None
  }

  /// Sibling object property coming after this one.
  pub fn next_property(&self) -> Option<CstObjectProp> {
    for sibling in self.next_siblings() {
      if let CstNode::Container(CstContainerNode::ObjectProp(prop)) = sibling {
        return Some(prop);
      }
    }
    None
  }

  /// Replaces this node with a new value.
  pub fn replace_with(self, key: &str, replacement: CstInputValue) -> Option<CstNode> {
    replace_with(self.into(), InsertValue::Property(key, replacement))
  }

  /// Removes the node from the JSON.
  pub fn remove(self) {
    remove_comma_separated(self.into())
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
  add_root_node_method!();
  add_parent_info_methods!();

  /// Object property name if it's a string literal.
  pub fn as_string_lit(&self) -> Option<&CstStringLit> {
    match self {
      ObjectPropName::String(n) => Some(n),
      ObjectPropName::Word(_) => None,
    }
  }

  /// Object property name if it's a word literal (no quotes).
  pub fn as_word_lit(&self) -> Option<&CstWordLit> {
    match self {
      ObjectPropName::String(_) => None,
      ObjectPropName::Word(n) => Some(n),
    }
  }

  /// Decoded value of the string.
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

impl CstArray {
  add_root_node_method!();

  fn new() -> Self {
    Self(CstValueInner::new(Vec::new()))
  }

  /// Elements of the array.
  pub fn elements(&self) -> Vec<CstNode> {
    self
      .0
      .borrow()
      .value
      .iter()
      .filter(|child| match child {
        CstNode::Container(_) => true,
        CstNode::Leaf(leaf) => match leaf {
          CstLeafNode::BooleanLit(_)
          | CstLeafNode::NullKeyword(_)
          | CstLeafNode::NumberLit(_)
          | CstLeafNode::StringLit(_)
          | CstLeafNode::WordLit(_) => true,
          CstLeafNode::Token(_) | CstLeafNode::Whitespace(_) | CstLeafNode::Newline(_) | CstLeafNode::Comment(_) => {
            false
          }
        },
      })
      .cloned()
      .collect()
  }

  /// Appends an element to the end of the array.
  pub fn append(&self, value: CstInputValue) {
    self.insert_or_append(None, value);
  }

  /// Inserts an element at the specified index.
  pub fn insert(&self, index: usize, value: CstInputValue) {
    self.insert_or_append(Some(index), value);
  }

  /// Ensures the array spans multiple lines.
  pub fn ensure_multiline(&self) {
    let children = self.children();
    if children.iter().any(|c| c.is_newline()) {
      return;
    }

    let indents = compute_indents(&self.clone().into());
    let child_indents = indents.indent();
    let container: CstContainerNode = self.clone().into();
    let newline_kind = self
      .root_node()
      .map(|r| r.newline_kind())
      .unwrap_or(CstNewlineKind::LineFeed);

    // insert a newline at the start of every part
    let children_len = children.len();
    let mut children = children.into_iter().skip(1).peekable().take(children_len - 2);
    let mut index = 1;
    while let Some(child) = children.next() {
      if child.is_whitespace() {
        child.remove();
        continue;
      } else {
        // insert a newline
        container.raw_insert_child(Some(&mut index), CstNewline::new(newline_kind).into());
        container.raw_insert_child(
          Some(&mut index),
          CstWhitespace::new(child_indents.current_indent.clone()).into(),
        );

        // current node
        index += 1;

        // consume the next tokens until the next comma
        let mut trailing_whitespace = Vec::new();
        for next_child in children.by_ref() {
          if next_child.is_whitespace() {
            trailing_whitespace.push(next_child);
          } else {
            index += 1 + trailing_whitespace.len();
            trailing_whitespace.clear();
            if next_child.token_char() == Some(',') {
              break;
            }
          }
        }

        for trailing_whitespace in trailing_whitespace {
          trailing_whitespace.remove();
        }
      }
    }

    // insert the last newline
    container.raw_insert_child(Some(&mut index), CstNewline::new(newline_kind).into());
    if !indents.current_indent.is_empty() {
      container.raw_insert_child(Some(&mut index), CstWhitespace::new(indents.current_indent).into());
    }
  }

  fn insert_or_append(&self, index: Option<usize>, value: CstInputValue) {
    insert_or_append_to_container(
      &CstContainerNode::Array(self.clone()),
      self.elements(),
      index,
      InsertValue::Value(value),
    )
  }

  /// Replaces this node with a new value.
  pub fn replace_with(self, replacement: CstInputValue) -> Option<CstNode> {
    replace_with(self.into(), InsertValue::Value(replacement))
  }

  /// Removes the node from the JSON.
  pub fn remove(self) {
    remove_comma_separated(self.into())
  }
}

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
  fn new(value: char) -> Self {
    Self(CstValueInner::new(value))
  }

  /// Sets the char value of the token.
  pub fn set_value(&self, value: char) {
    self.0.borrow_mut().value = value;
  }

  /// Char value of the token.
  pub fn value(&self) -> char {
    self.0.borrow().value
  }

  /// Removes the node from the JSON.
  pub fn remove(self) {
    Into::<CstNode>::into(self).remove_raw()
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
  fn new(value: String) -> Self {
    Self(CstValueInner::new(value))
  }

  /// Sets the whitespace value.
  pub fn set_value(&self, value: String) {
    self.0.borrow_mut().value = value;
  }

  /// Whitespace value of the node.
  pub fn value(&self) -> String {
    self.0.borrow().value.clone()
  }

  /// Removes the node from the JSON.
  pub fn remove(self) {
    Into::<CstNode>::into(self).remove_raw()
  }
}

impl Display for CstWhitespace {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0.borrow().value)
  }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CstNewlineKind {
  #[default]
  LineFeed,
  CarriageReturnLineFeed,
}

#[derive(Debug, Clone)]
pub struct CstNewline(Rc<RefCell<CstValueInner<CstNewlineKind>>>);

impl_leaf_methods!(CstNewline, Newline);

impl CstNewline {
  fn new(kind: CstNewlineKind) -> Self {
    Self(CstValueInner::new(kind))
  }

  /// Whether this is a line feed (LF) or carriage return line feed (CRLF).
  pub fn kind(&self) -> CstNewlineKind {
    self.0.borrow().value
  }

  /// Sets the newline kind.
  pub fn set_kind(&self, kind: CstNewlineKind) {
    self.0.borrow_mut().value = kind;
  }

  /// Removes the node from the JSON.
  pub fn remove(self) {
    Into::<CstNode>::into(self).remove_raw()
  }
}

impl Display for CstNewline {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.0.borrow().value {
      #[allow(clippy::write_with_newline)] // better to be explicit
      CstNewlineKind::LineFeed => write!(f, "\n"),
      CstNewlineKind::CarriageReturnLineFeed => write!(f, "\r\n"),
    }
  }
}

#[derive(Debug, Clone)]
pub struct CstComment(Rc<RefCell<CstValueInner<String>>>);

impl_leaf_methods!(CstComment, Comment);

impl CstComment {
  fn new(value: String) -> Self {
    Self(CstValueInner::new(value))
  }

  /// Whether this is a line comment.
  pub fn is_line_comment(&self) -> bool {
    self.0.borrow().value.starts_with("//")
  }

  /// Sets the raw value of the comment.
  ///
  /// This SHOULD include `//` or be surrounded in `/* ... */` or
  /// else you'll be inserting a syntax error.
  pub fn set_raw_value(&self, value: String) {
    self.0.borrow_mut().value = value;
  }

  /// Raw value of the comment including `//` or `/* ... */`.
  pub fn raw_value(&self) -> String {
    self.0.borrow().value.clone()
  }

  /// Removes the node from the JSON.
  pub fn remove(self) {
    if self.is_line_comment() {
      for node in self.previous_siblings() {
        if node.is_whitespace() {
          node.remove_raw();
        } else {
          if node.is_newline() {
            node.remove_raw();
          }
          break;
        }
      }
    }

    Into::<CstNode>::into(self).remove_raw()
  }
}

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
            container
              .raw_append_child(CstComment::new(self.text[token.range.start..token.range.end].to_string()).into());
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
      ast::Value::NumberLit(number_lit) => {
        container.raw_append_child(CstNumberLit::new(number_lit.value.to_string()).into())
      }
      ast::Value::BooleanLit(boolean_lit) => container.raw_append_child(CstBooleanLit::new(boolean_lit.value).into()),
      ast::Value::Object(object) => {
        let object = self.build_object(object);
        container.raw_append_child(object.into())
      }
      ast::Value::Array(array) => {
        let array = self.build_array(array);
        container.raw_append_child(array.into())
      }
      ast::Value::NullKeyword(_) => container.raw_append_child(CstNullKeyword::new().into()),
    }
  }

  fn build_object(&mut self, object: ast::Object<'_>) -> CstContainerNode {
    let container = CstContainerNode::Object(CstObject::new());
    let mut last_range_end = object.range.start;
    for prop in object.properties {
      self.scan_from_to(&container, last_range_end, prop.range.start);
      last_range_end = prop.range.end;
      let object_prop = self.build_object_prop(prop);
      container.raw_append_child(CstNode::Container(object_prop));
    }
    self.scan_from_to(&container, last_range_end, object.range.end);

    container
  }

  fn build_object_prop(&mut self, prop: ast::ObjectProp<'_>) -> CstContainerNode {
    let container = CstContainerNode::ObjectProp(CstObjectProp::new());
    let name_range = prop.name.range();
    let value_range = prop.value.range();

    match prop.name {
      ast::ObjectPropName::String(string_lit) => {
        self.build_string_lit(&container, string_lit);
      }
      ast::ObjectPropName::Word(word_lit) => {
        container.raw_append_child(CstWordLit::new(word_lit.value.to_string()).into());
      }
    }

    self.scan_from_to(&container, name_range.end, value_range.start);
    self.build_value(&container, prop.value);

    container
  }

  fn build_token(&self, container: &CstContainerNode, value: char) {
    container.raw_append_child(CstToken::new(value).into());
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
        container.raw_append_child(CstWhitespace::new(text.to_string()).into());
      }
    };
    while let Some((i, c)) = chars.next() {
      if c == '\r' && chars.peek().map(|(_, c)| *c) == Some('\n') {
        maybe_add_previous_text(last_found_index, i);
        container.raw_append_child(CstNewline::new(CstNewlineKind::CarriageReturnLineFeed).into());
        last_found_index = i + 2;
      } else if c == '\n' {
        maybe_add_previous_text(last_found_index, i);
        container.raw_append_child(CstNewline::new(CstNewlineKind::LineFeed).into());
        last_found_index = i + 1;
      }
    }

    maybe_add_previous_text(last_found_index, value.len());
  }

  fn build_string_lit(&self, container: &CstContainerNode, lit: ast::StringLit<'_>) {
    container.raw_append_child(CstStringLit::new(self.text[lit.range.start..lit.range.end].to_string()).into());
  }

  fn build_array(&mut self, array: ast::Array<'_>) -> CstContainerNode {
    let container = CstContainerNode::Array(CstArray::new());
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
}

fn remove_comma_separated(node: CstNode) {
  fn check_next_node_same_line(trailing_comma: &CstToken) -> bool {
    for sibling in trailing_comma.next_siblings() {
      match sibling {
        CstNode::Container(_) => return true,
        CstNode::Leaf(n) => match n {
          CstLeafNode::BooleanLit(_)
          | CstLeafNode::NullKeyword(_)
          | CstLeafNode::NumberLit(_)
          | CstLeafNode::StringLit(_)
          | CstLeafNode::WordLit(_)
          | CstLeafNode::Token(_) => return true,
          CstLeafNode::Whitespace(_) | CstLeafNode::Comment(_) => {
            // keep going
          }
          CstLeafNode::Newline(_) => return false,
        },
      }
    }

    true
  }

  let parent = node.parent();
  let trailing_comma = node.trailing_comma();
  let is_in_array_or_obj = parent
    .as_ref()
    .map(|p| matches!(p, CstContainerNode::Array(_) | CstContainerNode::Object(_)))
    .unwrap_or(false);
  let remove_up_to_next_line = trailing_comma
    .as_ref()
    .map(|c| !check_next_node_same_line(c))
    .unwrap_or(true);

  for previous in node.previous_siblings() {
    if previous.is_trivia() && !previous.is_newline() {
      previous.remove_raw();
    } else {
      break;
    }
  }

  let mut found_newline = false;
  let mut next_siblings = node.next_siblings();

  // remove up to the trailing comma
  if trailing_comma.is_some() {
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
  } else if is_in_array_or_obj {
    if let Some(previous_comma) = node.previous_siblings().find(|n| n.is_comma()) {
      previous_comma.remove();
    }
  }

  // remove up to the newline
  if remove_up_to_next_line && !found_newline {
    let mut next_siblings = node.next_siblings().peekable();
    while let Some(sibling) = next_siblings.next() {
      if sibling.is_trivia() {
        if sibling.is_newline() {
          sibling.remove_raw();
          break;
        } else if sibling.is_whitespace()
          && next_siblings
            .peek()
            .map(|n| !n.is_whitespace() && !n.is_newline() && !n.is_comment())
            .unwrap_or(false)
        {
          break;
        }
        sibling.remove_raw();
      } else {
        break;
      }
    }
  }

  node.remove_raw();

  if let Some(parent) = parent {
    match parent {
      CstContainerNode::Root(n) => {
        if n.children().iter().all(|c| c.is_whitespace() || c.is_newline()) {
          n.clear_children();
        }
      }
      CstContainerNode::Object(_) | CstContainerNode::Array(_) => {
        let children = parent.children();
        if children
          .iter()
          .skip(1)
          .take(children.len() - 2)
          .all(|c| c.is_whitespace() || c.is_newline())
        {
          for c in children {
            if c.is_whitespace() || c.is_newline() {
              c.remove();
            }
          }
        }
      }
      CstContainerNode::ObjectProp(_) => {}
    }
  }
}

fn indent_text(node: &CstNode) -> Option<String> {
  let mut last_whitespace: Option<String> = None;
  for previous_sibling in node.previous_siblings() {
    match previous_sibling {
      CstNode::Container(_) => return None,
      CstNode::Leaf(leaf) => match leaf {
        CstLeafNode::Newline(_) => {
          return last_whitespace;
        }
        CstLeafNode::Whitespace(whitespace) => {
          last_whitespace = match last_whitespace {
            Some(last_whitespace) => Some(format!("{}{}", whitespace.value(), last_whitespace)),
            None => Some(whitespace.value()),
          };
        }
        CstLeafNode::Comment(_) => {
          last_whitespace = None;
        }
        _ => return None,
      },
    }
  }
  last_whitespace
}

fn uses_trailing_commas(node: CstNode) -> bool {
  let node = match node {
    CstNode::Container(node) => node,
    CstNode::Leaf(_) => return false,
  };
  let mut pending_nodes: VecDeque<CstContainerNode> = VecDeque::from([node.clone()]);
  while let Some(node) = pending_nodes.pop_front() {
    let children = node.children();
    if !node.is_root() {
      if let Some(object) = node.as_object() {
        if children.iter().any(|c| c.is_whitespace()) {
          let properties = object.properties();
          if let Some(last_property) = properties.last() {
            return last_property.trailing_comma().is_some();
          }
        }
      } else if let Some(object) = node.as_array() {
        if children.iter().any(|c| c.is_whitespace()) {
          let elements = object.elements();
          if let Some(last_property) = elements.last() {
            return last_property.trailing_comma().is_some();
          }
        }
      }
    }

    for child in children {
      if let CstNode::Container(child) = child {
        pending_nodes.push_back(child);
      }
    }
  }

  false // default to false
}

fn replace_with(node: CstNode, replacement: InsertValue) -> Option<CstNode> {
  let mut child_index = node.child_index();
  let parent = node.parent()?;
  let indents = compute_indents(&parent.clone().into());
  let style_info = StyleInfo {
    newline_kind: parent.root_node().map(|r| r.newline_kind()).unwrap_or_default(),
    uses_trailing_commas: uses_trailing_commas(parent.clone().into()),
  };
  parent.remove_child_set_no_parent(child_index);
  parent.raw_insert_value_with_internal_indent(Some(&mut child_index), replacement, &style_info, &indents);
  parent.child_at_index(child_index - 1)
}

enum InsertValue<'a> {
  Value(CstInputValue),
  Property(&'a str, CstInputValue),
}

fn insert_or_append_to_container(
  container: &CstContainerNode,
  elements: Vec<CstNode>,
  index: Option<usize>,
  value: InsertValue,
) {
  fn has_separating_newline(siblings: impl Iterator<Item = CstNode>) -> bool {
    for sibling in siblings {
      if sibling.is_newline() {
        return true;
      } else if sibling.is_trivia() {
        continue;
      } else {
        break;
      }
    }
    false
  }

  trim_inner_start_and_end_blanklines(container);

  let children = container.children();
  let index = index.unwrap_or(elements.len());
  let index = std::cmp::min(index, elements.len());
  let next_node = elements.get(index);
  let previous_node = if index == 0 { None } else { elements.get(index - 1) };
  let style_info = StyleInfo {
    newline_kind: container.root_node().map(|r| r.newline_kind()).unwrap_or_default(),
    uses_trailing_commas: uses_trailing_commas(container.clone().into()),
  };
  let indents = compute_indents(&container.clone().into());
  let child_indents = elements
    .first()
    .map(compute_indents)
    .unwrap_or_else(|| indents.indent());
  let has_newline = children.iter().any(|child| child.is_newline());
  let force_multiline = has_newline
    || match &value {
      InsertValue::Value(v) => v.force_multiline(),
      InsertValue::Property(..) => true,
    };
  let mut insert_index: usize;
  if let Some(previous_node) = previous_node {
    if previous_node.trailing_comma().is_none() {
      let mut index = previous_node.child_index() + 1;
      container.raw_insert_child(Some(&mut index), CstToken::new(',').into());
    }

    let trailing_comma: CstNode = previous_node.trailing_comma().unwrap().into();
    insert_index = trailing_comma
      .trailing_comments_same_line()
      .last()
      .map(|t| t.child_index())
      .unwrap_or_else(|| trailing_comma.child_index())
      + 1;
    if force_multiline {
      container.raw_insert_children(
        Some(&mut insert_index),
        vec![
          CstNewline::new(style_info.newline_kind).into(),
          CstStringLit::new(child_indents.current_indent.clone()).into(),
        ],
      );
      container.raw_insert_value_with_internal_indent(Some(&mut insert_index), value, &style_info, &child_indents);
    } else {
      container.raw_insert_child(Some(&mut insert_index), CstWhitespace::new(" ".to_string()).into());
      container.raw_insert_value_with_internal_indent(Some(&mut insert_index), value, &style_info, &child_indents);
    }
  } else {
    insert_index = if elements.is_empty() {
      children
        .iter()
        .rev()
        .skip(1)
        .take_while(|t| t.is_whitespace() || t.is_newline())
        .last()
        .unwrap_or_else(|| children.last().unwrap())
        .child_index()
    } else {
      children.first().unwrap().child_index() + 1
    };
    if force_multiline {
      container.raw_insert_children(
        Some(&mut insert_index),
        vec![
          CstNewline::new(style_info.newline_kind).into(),
          CstStringLit::new(child_indents.current_indent.clone()).into(),
        ],
      );
      container.raw_insert_value_with_internal_indent(Some(&mut insert_index), value, &style_info, &child_indents);
      if next_node.is_none()
        && !has_separating_newline(container.child_at_index(insert_index - 1).unwrap().next_siblings())
      {
        container.raw_insert_children(
          Some(&mut insert_index),
          vec![
            CstNewline::new(style_info.newline_kind).into(),
            CstStringLit::new(indents.current_indent.clone()).into(),
          ],
        );
      }
    } else {
      container.raw_insert_value_with_internal_indent(Some(&mut insert_index), value, &style_info, &child_indents);
    }
  }

  if next_node.is_some() {
    container.raw_insert_children(Some(&mut insert_index), vec![CstToken::new(',').into()]);

    if force_multiline {
      let comma_token = container.child_at_index(insert_index - 1).unwrap();
      if !has_separating_newline(comma_token.next_siblings()) {
        container.raw_insert_children(
          Some(&mut insert_index),
          vec![
            CstNewline::new(style_info.newline_kind).into(),
            CstStringLit::new(indents.current_indent.clone()).into(),
          ],
        );
      }
    } else {
      container.raw_insert_child(Some(&mut insert_index), CstWhitespace::new(" ".to_string()).into());
    }
  } else if style_info.uses_trailing_commas && force_multiline {
    container.raw_insert_children(Some(&mut insert_index), vec![CstToken::new(',').into()]);
  }
}

fn trim_inner_start_and_end_blanklines(node: &CstContainerNode) {
  fn remove_blank_lines_after_first(children: &mut Peekable<impl Iterator<Item = CstNode>>) {
    // try to find the first newline
    for child in children.by_ref() {
      if child.is_whitespace() {
        // keep searching
      } else if child.is_newline() {
        break; // found
      } else {
        return; // stop, no leading blank lines
      }
    }

    let mut pending = Vec::new();
    for child in children.by_ref() {
      if child.is_whitespace() {
        pending.push(child);
      } else if child.is_newline() {
        child.remove();
        for child in pending.drain(..) {
          child.remove();
        }
      } else {
        break;
      }
    }
  }

  let children = node.children();
  let len = children.len();

  if len < 2 {
    return; // should never happen because this should only be called for array and object
  }

  // remove blank lines from the front and back
  let mut children = children.into_iter().skip(1).take(len - 2).peekable();
  remove_blank_lines_after_first(&mut children);
  let mut children = children.rev().peekable();
  remove_blank_lines_after_first(&mut children);
}

#[derive(Debug)]
struct Indents {
  current_indent: String,
  single_indent: String,
}

impl Indents {
  pub fn indent(&self) -> Indents {
    Indents {
      current_indent: format!("{}{}", self.current_indent, self.single_indent),
      single_indent: self.single_indent.clone(),
    }
  }
}

fn compute_indents(node: &CstNode) -> Indents {
  let mut indent_level = 0;
  let mut stored_last_indent = node.indent_text();
  let mut ancestors = node.ancestors().peekable();

  while ancestors.peek().and_then(|p| p.as_object_prop()).is_some() {
    ancestors.next();
  }

  while let Some(ancestor) = ancestors.next() {
    if ancestor.is_root() {
      break;
    }

    if ancestors.peek().and_then(|p| p.as_object_prop()).is_some() {
      continue;
    }

    indent_level += 1;

    if let Some(indent_text) = ancestor.indent_text() {
      match stored_last_indent {
        Some(last_indent) => {
          if let Some(single_indent_text) = last_indent.strip_prefix(&indent_text) {
            return Indents {
              current_indent: format!("{}{}", last_indent, single_indent_text.repeat(indent_level - 1)),
              single_indent: single_indent_text.to_string(),
            };
          }
          stored_last_indent = None;
        }
        None => {
          stored_last_indent = Some(indent_text);
        }
      }
    } else {
      stored_last_indent = None;
    }
  }

  if indent_level == 1 {
    if let Some(indent_text) = node.indent_text() {
      return Indents {
        current_indent: indent_text.clone(),
        single_indent: indent_text,
      };
    }
  }

  // try to discover the single indent level by looking at the root node's children
  if let Some(root_value) = node.root_node().and_then(|r| r.root_value()) {
    for child in root_value.children() {
      if let Some(single_indent) = child.indent_text() {
        return Indents {
          current_indent: single_indent.repeat(indent_level),
          single_indent,
        };
      }
    }
  }

  // assume two space indentation
  let single_indent = "  ";
  Indents {
    current_indent: single_indent.repeat(indent_level),
    single_indent: single_indent.to_string(),
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

  use crate::cst::CstInputValue;
  use crate::value;

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
      let root = build_cst(text);
      assert_eq!(root.single_indent_text(), Some(expected.to_string()), "Text: {}", text);
    }
  }

  #[test]
  fn modify_values() {
    let cst = build_cst(
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
      let prop = root_obj.get("value").unwrap();
      prop
        .value()
        .unwrap()
        .as_number_lit()
        .unwrap()
        .set_raw_value("10".to_string());
      assert!(prop.trailing_comma().is_some());
      assert!(prop.previous_property().is_none());
      assert_eq!(
        prop.next_property().unwrap().name().unwrap().decoded_value().unwrap(),
        "value2"
      );
      assert_eq!(prop.indent_text().unwrap(), "    ");
    }
    {
      let prop = root_obj.get("value2").unwrap();
      prop
        .value()
        .unwrap()
        .as_string_lit()
        .unwrap()
        .set_raw_value("\"5\"".to_string());
      assert!(prop.trailing_comma().is_some());
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
      let prop = root_obj.get("value3").unwrap();
      prop.value().unwrap().as_boolean_lit().unwrap().set_value(false);
      prop
        .name()
        .unwrap()
        .as_word_lit()
        .unwrap()
        .set_raw_value("value4".to_string());
      assert!(prop.trailing_comma().is_none());
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
      let cst = build_cst(json);
      let root_value = cst.root_value().unwrap();
      let root_obj = root_value.as_object().unwrap();
      let prop = root_obj.get(prop_name).unwrap();
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

    run_test("value", r#"{ "value": 5 }"#, r#"{}"#);
    run_test("value", r#"{ "value": 5, "value2": 6 }"#, r#"{ "value2": 6 }"#);
    run_test("value2", r#"{ "value": 5, "value2": 6 }"#, r#"{ "value": 5 }"#);
  }

  #[test]
  fn insert_properties() {
    fn run_test(index: usize, prop_name: &str, value: CstInputValue, json: &str, expected: &str) {
      let cst = build_cst(json);
      let root_value = cst.root_value().unwrap();
      let root_obj = root_value.as_object().unwrap();
      root_obj.insert(index, prop_name, value);
      assert_eq!(cst.to_string(), expected, "Initial text: {}", json);
    }

    run_test(
      0,
      "propName",
      value!([1]),
      r#"{}"#,
      r#"{
  "propName": [1]
}"#,
    );

    // inserting before first prop
    run_test(
      0,
      "value0",
      value!([1]),
      r#"{
    "value1": 5
}"#,
      r#"{
    "value0": [1],
    "value1": 5
}"#,
    );

    // inserting before first prop with leading comment
    run_test(
      0,
      "value0",
      value!([1]),
      r#"{
    // some comment
    "value1": 5
}"#,
      r#"{
    "value0": [1],
    // some comment
    "value1": 5
}"#,
    );

    // inserting after last prop with trailing comment
    run_test(
      1,
      "value1",
      value!({
        "value": 1
      }),
      r#"{
    "value0": 5 // comment
}"#,
      r#"{
    "value0": 5, // comment
    "value1": {
        "value": 1
    }
}"#,
    );

    // maintain trailing comma
    run_test(
      1,
      "propName",
      value!(true),
      r#"{
  "value": 4,
}"#,
      r#"{
  "value": 4,
  "propName": true,
}"#,
    );
  }

  #[test]
  fn remove_array_elements() {
    fn run_test(index: usize, json: &str, expected: &str) {
      let cst = build_cst(json);
      let root_value = cst.root_value().unwrap();
      let root_array = root_value.as_array().unwrap();
      let element = root_array.elements().get(index).unwrap().clone();
      element.remove();
      assert_eq!(cst.to_string(), expected);
    }

    run_test(
      0,
      r#"[
      1,
]"#,
      r#"[]"#,
    );
    run_test(
      0,
      r#"[
      1,
      2,
]"#,
      r#"[
      2,
]"#,
    );
    run_test(
      0,
      r#"[
      1,
      2,
]"#,
      r#"[
      2,
]"#,
    );

    run_test(
      1,
      r#"[
      1, // other comment
      2, // trailing comment
]"#,
      r#"[
      1, // other comment
]"#,
    );

    run_test(
      1,
      r#"[
      1, // comment
      2
]"#,
      r#"[
      1 // comment
]"#,
    );

    run_test(1, r#"[1, 2]"#, r#"[1]"#);
    run_test(1, r#"[ 1, 2 /* test */ ]"#, r#"[ 1 ]"#);
    run_test(1, r#"[1, /* test */ 2]"#, r#"[1]"#);
    run_test(
      1,
      r#"[1 /* a */, /* b */ 2 /* c */, /* d */ true]"#,
      r#"[1 /* a */, /* d */ true]"#,
    );
  }

  #[test]
  fn insert_array_element() {
    #[track_caller]
    fn run_test(index: usize, value: CstInputValue, json: &str, expected: &str) {
      let cst = build_cst(json);
      let root_value = cst.root_value().unwrap();
      let root_array = root_value.as_array().unwrap();
      root_array.insert(index, value);
      assert_eq!(cst.to_string(), expected, "Initial text: {}", json);
    }

    run_test(0, value!([1]), r#"[]"#, r#"[[1]]"#);
    run_test(0, value!([1, true, false, {}]), r#"[]"#, r#"[[1, true, false, {}]]"#);
    run_test(0, value!(10), r#"[]"#, r#"[10]"#);
    run_test(0, value!(10), r#"[1]"#, r#"[10, 1]"#);
    run_test(1, value!(10), r#"[1]"#, r#"[1, 10]"#);
    run_test(
      0,
      value!(10),
      r#"[
    1
]"#,
      r#"[
    10,
    1
]"#,
    );
    run_test(
      0,
      value!(10),
      r#"[
    /* test */ 1
]"#,
      r#"[
    10,
    /* test */ 1
]"#,
    );

    run_test(
      0,
      value!({
        "value": 1,
      }),
      r#"[]"#,
      r#"[
  {
    "value": 1
  }
]"#,
    );

    // only comment
    run_test(
      0,
      value!({
        "value": 1,
      }),
      r#"[
    // comment
]"#,
      r#"[
    // comment
    {
        "value": 1
    }
]"#,
    );

    // blank line
    run_test(
      0,
      value!({
        "value": 1,
      }),
      r#"[

]"#,
      r#"[
  {
    "value": 1
  }
]"#,
    );
  }

  #[test]
  fn insert_array_element_trailing_commas() {
    let cst = build_cst(
      r#"{
    "prop": [
      1,
      2,
    ]
}"#,
    );
    cst
      .ensure_object_value()
      .unwrap()
      .get_array("prop")
      .unwrap()
      .append(value!(3));
    assert_eq!(
      cst.to_string(),
      r#"{
    "prop": [
      1,
      2,
      3,
    ]
}"#
    );
  }

  #[test]
  fn remove_comment() {
    fn run_test(json: &str, expected: &str) {
      let cst = build_cst(json);
      let root_value = cst.root_value().unwrap();
      let root_obj = root_value.as_object().unwrap();
      root_obj
        .children()
        .into_iter()
        .filter_map(|c| c.as_comment().cloned())
        .next()
        .unwrap()
        .remove();
      assert_eq!(cst.to_string(), expected);
    }

    run_test(
      r#"{
    "value": 5,
    // comment
    "value2": "hello",
    value3: true
}"#,
      r#"{
    "value": 5,
    "value2": "hello",
    value3: true
}"#,
    );

    run_test(
      r#"{
    "value": 5,  // comment
    "value2": "hello",
    value3: true
}"#,
      r#"{
    "value": 5,
    "value2": "hello",
    value3: true
}"#,
    );
  }

  #[test]
  fn ensure_object_value() {
    // existing
    {
      let cst = build_cst(r#"{ "value": 1 }"#);
      let obj = cst.ensure_object_value().unwrap();
      assert!(obj.get("value").is_some());
    }
    // empty file
    {
      let cst = build_cst(r#""#);
      cst.ensure_object_value().unwrap();
      assert_eq!(cst.to_string(), "{}\n");
    }
    // comment
    {
      let cst = build_cst("// Copyright something");
      cst.ensure_object_value().unwrap();
      assert_eq!(cst.to_string(), "// Copyright something\n{}\n");
    }
    // comment and newline
    {
      let cst = build_cst("// Copyright something\n");
      cst.ensure_object_value().unwrap();
      assert_eq!(cst.to_string(), "// Copyright something\n{}\n");
    }
  }

  #[test]
  fn array_ensure_multiline() {
    // empty
    {
      let cst = build_cst(r#"[]"#);
      cst.root_value().unwrap().as_array().unwrap().ensure_multiline();
      assert_eq!(cst.to_string(), "[\n]");
    }
    // whitespace only
    {
      let cst = build_cst(r#"[   ]"#);
      cst.root_value().unwrap().as_array().unwrap().ensure_multiline();
      assert_eq!(cst.to_string(), "[\n]");
    }
    // comments only
    {
      let cst = build_cst(r#"[  /* test */  ]"#);
      cst.root_value().unwrap().as_array().unwrap().ensure_multiline();
      assert_eq!(cst.to_string(), "[\n  /* test */\n]");
    }
    // elements
    {
      let cst = build_cst(r#"[  1,   2, /* test */ 3  ]"#);
      cst.root_value().unwrap().as_array().unwrap().ensure_multiline();
      assert_eq!(
        cst.to_string(),
        r#"[
  1,
  2,
  /* test */ 3
]"#
      );
    }
    // elements deep
    {
      let cst = build_cst(
        r#"{
  "prop": {
    "value": [  1,   2, /* test */ 3  ]
  }
}"#,
      );
      cst
        .root_value()
        .unwrap()
        .as_object()
        .unwrap()
        .get("prop")
        .unwrap()
        .value()
        .unwrap()
        .as_object()
        .unwrap()
        .get("value")
        .unwrap()
        .value()
        .unwrap()
        .as_array()
        .unwrap()
        .ensure_multiline();
      assert_eq!(
        cst.to_string(),
        r#"{
  "prop": {
    "value": [
      1,
      2,
      /* test */ 3
    ]
  }
}"#
      );
    }
  }

  fn build_cst(text: &str) -> CstRootNode {
    CstRootNode::parse(text, &crate::ParseOptions::default()).unwrap()
  }
}
