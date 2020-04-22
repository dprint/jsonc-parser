# jsonc-parser

[![](https://img.shields.io/crates/v/jsonc-parser.svg)](https://crates.io/crates/jsonc-parser)

JSONC to AST parser implemented in Rust.

## Example

```rust
use jsonc_parser::parse_text;

let parse_result = parse_text(r#"{ "test": 5 } // test"#);
// ...inspect parse_result for value, tokens, and comments here...
```
