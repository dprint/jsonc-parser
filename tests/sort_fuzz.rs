//! Checks the CST sorting against randomly generated JSONC.
//!
//! Reordering has to move each element's comments and commas with it without ever changing what
//! the document holds, and the ways to get that wrong are mostly odd trivia placements rather than
//! odd values. So the generator varies the trivia in every slot it can appear in and the values
//! barely at all, then asserts the invariants that must hold however the pieces land.

#![cfg(feature = "cst")]

use jsonc_parser::ParseOptions;
use jsonc_parser::cst::CstArray;
use jsonc_parser::cst::CstObject;
use jsonc_parser::cst::CstRootNode;

#[test]
fn sorting_generated_documents_preserves_them() {
  let mut random = Random::new(0x5eed_1234_9abc_def0);
  for _ in 0..20_000 {
    // the options only change which trivia travels and how far an element may move, so every
    // combination has to hold the same invariants
    let options = Options {
      pin_headers: random.chance(2),
      within_groups: random.chance(2),
    };
    check(&mut random, Shape::Object, options);
    check(&mut random, Shape::Array, options);
  }
}

fn check(random: &mut Random, shape: Shape, options: Options) {
  let text = generate(random, shape);
  let Ok(root) = CstRootNode::parse(&text, &ParseOptions::default()) else {
    // the generator is allowed to produce something the parser rejects; nothing to sort then
    return;
  };
  let before = contents(&root);
  let comments_before = comments(&text);

  match shape {
    Shape::Object => {
      let Some(object) = root.object_value() else {
        return;
      };
      sort_properties(&object, options);
    }
    Shape::Array => {
      let Some(array) = root.array_value() else {
        return;
      };
      sort_elements(&array, options);
    }
  }

  let sorted = root.to_string();
  let reparsed = CstRootNode::parse(&sorted, &ParseOptions::default())
    .unwrap_or_else(|err| panic!("did not re-parse: {err}\n--- input ---\n{text}\n--- output ---\n{sorted}"));

  let after = contents(&reparsed);
  assert_eq!(
    sorted_lines(&before),
    sorted_lines(&after),
    "contents changed\n--- input ---\n{text}\n--- output ---\n{sorted}"
  );
  // Only the key decides the order, and members sharing one keep the order they were written
  // in. Grouping deliberately leaves the container unsorted as a whole, so this only holds when
  // the sort was free to move a member anywhere.
  if !options.within_groups {
    assert!(
      after.windows(2).all(|pair| pair[0].0 <= pair[1].0),
      "not in order\n--- input ---\n{text}\n--- output ---\n{sorted}"
    );
  }
  assert_eq!(
    keyed_order(&before),
    keyed_order(&after),
    "members sharing a key changed order\n--- input ---\n{text}\n--- output ---\n{sorted}"
  );
  assert_eq!(
    comments_before,
    comments(&sorted),
    "comments changed\n--- input ---\n{text}\n--- output ---\n{sorted}"
  );

  // sorting what is already sorted leaves it alone
  match shape {
    Shape::Object => sort_properties(&reparsed.object_value().unwrap(), options),
    Shape::Array => sort_elements(&reparsed.array_value().unwrap(), options),
  }
  assert_eq!(
    reparsed.to_string(),
    sorted,
    "not idempotent\n--- input ---\n{text}\n--- output ---\n{sorted}"
  );
}

#[derive(Clone, Copy)]
struct Options {
  pin_headers: bool,
  within_groups: bool,
}

fn sort_properties(object: &CstObject, options: Options) {
  let mut sort = object.sort_properties();
  if options.pin_headers {
    sort = sort.pin_comment_headers();
  }
  if options.within_groups {
    sort = sort.within_groups();
  }
  sort.by_key(|prop| prop.decoded_name());
}

fn sort_elements(array: &CstArray, options: Options) {
  let mut sort = array.sort_elements();
  if options.pin_headers {
    sort = sort.pin_comment_headers();
  }
  if options.within_groups {
    sort = sort.within_groups();
  }
  sort.by_key(|element| element.to_string());
}

#[derive(Clone, Copy)]
enum Shape {
  Object,
  Array,
}

/// The sort key and value of each member, which reordering must leave untouched as a set.
fn contents(root: &CstRootNode) -> Vec<(String, String)> {
  if let Some(object) = root.object_value() {
    object
      .properties()
      .iter()
      .map(|prop| {
        let value = prop.value().map(|v| v.to_string()).unwrap_or_default();
        (prop.decoded_name().unwrap_or_default(), value.trim().to_string())
      })
      .collect()
  } else if let Some(array) = root.array_value() {
    array
      .elements()
      .iter()
      .map(|e| (e.to_string(), e.to_string()))
      .collect()
  } else {
    Vec::new()
  }
}

fn sorted_lines(values: &[(String, String)]) -> Vec<(String, String)> {
  let mut values = values.to_vec();
  values.sort();
  values
}

/// The members grouped under their key, in the order they appear.
///
/// A stable sort never reorders members sharing a key, so this has to come out the same before and
/// after however they were interleaved to begin with.
fn keyed_order(values: &[(String, String)]) -> Vec<(String, Vec<String>)> {
  let mut grouped: Vec<(String, Vec<String>)> = Vec::new();
  for (key, value) in values {
    match grouped.iter_mut().find(|(existing, _)| existing == key) {
      Some((_, values)) => values.push(value.clone()),
      None => grouped.push((key.clone(), vec![value.clone()])),
    }
  }
  grouped.sort_by(|left, right| left.0.cmp(&right.0));
  grouped
}

/// Every comment in the text, which reordering may move but never drop, merge, or invent.
fn comments(text: &str) -> Vec<String> {
  let mut comments = Vec::new();
  let bytes = text.as_bytes();
  let mut index = 0;
  let mut in_string = false;
  while index < bytes.len() {
    match bytes[index] {
      b'\\' if in_string => index += 1,
      b'"' => in_string = !in_string,
      b'/' if !in_string && index + 1 < bytes.len() && bytes[index + 1] == b'/' => {
        let start = index;
        while index < bytes.len() && bytes[index] != b'\n' && bytes[index] != b'\r' {
          index += 1;
        }
        comments.push(text[start..index].trim_end().to_string());
        continue;
      }
      b'/' if !in_string && index + 1 < bytes.len() && bytes[index + 1] == b'*' => {
        let start = index;
        index += 2;
        while index + 1 < bytes.len() && !(bytes[index] == b'*' && bytes[index + 1] == b'/') {
          index += 1;
        }
        index = (index + 2).min(bytes.len());
        comments.push(text[start..index].to_string());
        continue;
      }
      _ => {}
    }
    index += 1;
  }
  comments.sort();
  comments
}

fn generate(random: &mut Random, shape: Shape) -> String {
  let newline = if random.chance(4) { "\r\n" } else { "\n" };
  let multiline = random.chance(2);
  let member_count = 2 + random.below(4);
  let (open, close) = match shape {
    Shape::Object => ('{', '}'),
    Shape::Array => ('[', ']'),
  };

  let mut text = String::new();
  text.push(open);
  text.push_str(&trivia(random, newline, multiline, true));
  for index in 0..member_count {
    text.push_str(&trivia(random, newline, multiline, false));
    if multiline {
      text.push_str("  ");
    }
    match shape {
      Shape::Object => {
        text.push_str(&format!("\"{}\"", name(random, index)));
        text.push(':');
        text.push(' ');
      }
      Shape::Array => {}
    }
    text.push_str(&value(random, index));
    let last = index + 1 == member_count;
    if !last || random.chance(3) {
      // a comma sometimes lands after the trailing trivia, which is legal and worth exercising
      if random.chance(6) {
        text.push_str(&trailing_trivia(random, newline));
        text.push(',');
      } else {
        text.push(',');
        text.push_str(&trailing_trivia(random, newline));
      }
    } else {
      text.push_str(&trailing_trivia(random, newline));
    }
  }
  text.push_str(&trivia(random, newline, multiline, false));
  text.push(close);
  text
}

/// Trivia written above a member: line breaks, blank lines, and comments on their own line.
fn trivia(random: &mut Random, newline: &str, multiline: bool, after_open: bool) -> String {
  let mut text = String::new();
  if after_open && random.chance(4) {
    text.push_str(" // about the whole thing");
    text.push_str(newline);
    return text;
  }
  if multiline {
    text.push_str(newline);
    if random.chance(4) {
      text.push_str(newline);
    }
    if random.chance(3) {
      text.push_str("  ");
      text.push_str(if random.chance(2) {
        "// written above"
      } else {
        "/* written above */"
      });
      text.push_str(newline);
    }
  } else if !after_open {
    text.push(' ');
    if random.chance(5) {
      text.push_str("/* between */ ");
    }
  }
  text
}

/// Trivia written after a member on its own line.
fn trailing_trivia(random: &mut Random, newline: &str) -> String {
  if random.chance(4) {
    let mut text = String::from(" // trailing");
    text.push_str(newline);
    text
  } else if random.chance(6) {
    String::from(" /* trailing */")
  } else {
    String::new()
  }
}

fn name(random: &mut Random, index: usize) -> String {
  match random.below(4) {
    0 => format!("key{}", random.below(5)),
    1 => format!("\\u006bey{}", index),
    2 => format!("Key{}", random.below(5)),
    _ => format!("key{}", index),
  }
}

fn value(random: &mut Random, index: usize) -> String {
  match random.below(5) {
    0 => format!("{}", index),
    1 => format!("\"value{}\"", index),
    2 => String::from("{ \"nested\": true }"),
    3 => String::from("[1, 2]"),
    _ => String::from("null"),
  }
}

/// A tiny deterministic generator, so a failure can be reproduced from the seed alone.
struct Random(u64);

impl Random {
  fn new(seed: u64) -> Self {
    Self(seed)
  }

  fn next(&mut self) -> u64 {
    self.0 ^= self.0 << 13;
    self.0 ^= self.0 >> 7;
    self.0 ^= self.0 << 17;
    self.0
  }

  fn below(&mut self, bound: u64) -> usize {
    (self.next() % bound) as usize
  }

  fn chance(&mut self, one_in: u64) -> bool {
    self.next() % one_in == 0
  }
}
