use std::{fs::read_to_string, io::Result, time::Instant};
fn main() -> Result<()> {
    let content = read_to_string("../test.json")?;
    let start = Instant::now();
    let _result = jsonc_parser::parse_to_ast(&content, &jsonc_parser::ParseOptions::default());
    println!("{:?}", start.elapsed());
    Ok(())
}