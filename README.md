# jsonc-parser

[![](https://img.shields.io/crates/v/jsonc-parser.svg)](https://crates.io/crates/jsonc-parser)

JSONC parser implemented in Rust.

## Example

To a simple `JsonValue`:

```rs
use jsonc_parser::parse_to_value;

let json_value = parse_to_value(r#"{ "test": 5 } // test"#, &Default::default())?;
// check the json_value here
```

Or an AST:

```rs
use jsonc_parser::parse_to_ast;
use jsonc_parser::CollectOptions;

let parse_result = parse_to_ast(r#"{ "test": 5 } // test"#, &CollectOptions {
    comments: true, // include comments in result
    tokens: true, // include tokens in result
}, &Default::default())?;
// ...inspect parse_result for value, tokens, and comments here...
```

Or a CST (when enabling the `cst` feature), which provides a first class manipulation API:

```rs
use jsonc_parser::cst::CstRootNode;
use jsonc_parser::ParseOptions;
use jsonc_parser::value;

let json_text = r#"{
  // comment
  "data": 123
}"#;

let root = CstRootNode::parse(json_text, &ParseOptions::default()).unwrap();
let root_obj = root.root_value().unwrap().as_object().unwrap();

root_obj.get("data").unwrap().set_value(value!({
  "nested": true
}));
root_obj.append("new_key", value!([456, 789, false]));

assert_eq!(root.to_string(), r#"{
  // comment
  "data": {
    "nested": true,
  },
  "new_key": [456, 789, false]
}"#);
```

## Serde

If you enable the `"serde"` feature as follows:

```toml
# in Cargo.toml
jsonc-parser = { version = "...", features = ["serde"] }
```

Then you can use the `parse_to_serde_value` function to get a `serde_json::Value`:

```rs
use jsonc_parser::parse_to_serde_value;

let json_value = parse_to_serde_value(r#"{ "test": 5 } // test"#, &Default::default())?;
```

Alternatively, use `parse_to_ast` then call `.into()` (ex. `let value: serde_json::Value = ast.into();`).

## Parse Strictly as JSON

Provide `ParseOptions` and set all the options to false:

```rs
use jsonc_parser::parse_to_value;
use jsonc_parser::ParseOptions;

let json_value = parse_to_value(text, &ParseOptions {
  allow_comments: false,
  allow_loose_object_property_names: false,
  allow_trailing_commas: false,
})?;
```
