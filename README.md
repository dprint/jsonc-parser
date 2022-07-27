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

Or use the "serde" feature:

```toml
# in Cargo.toml
jsonc-parser = { version = "...", features = ["serde"] }
```

```rs
use jsonc_parser::parse_to_serde_value;

let json_value = parse_to_serde_value(r#"{ "test": 5 } // test"#, &Default::default())?;
```

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
