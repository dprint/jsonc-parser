# jsonc-parser

[![](https://img.shields.io/crates/v/jsonc-parser.svg)](https://crates.io/crates/jsonc-parser)

JSONC parser implemented in Rust.

## Example

To a simple `JsonValue`:

```rust
use jsonc_parser::parse_to_value;

let json_value = parse_to_value(r#"{ "test": 5 } // test"#)?;
// check the json_value here
```

Or an AST:

```rust
use jsonc_parser::{parse_to_ast, ParseOptions};

let parse_result = parse_to_ast(r#"{ "test": 5 } // test"#, &ParseOptions {
    comments: true, // include comments in result
    tokens: true, // include tokens in result
})?;
// ...inspect parse_result for value, tokens, and comments here...
```
