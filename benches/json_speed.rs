use criterion::{criterion_group, criterion_main, Criterion};
use jsonc_parser::{parse_to_ast, parse_to_value, ParseOptions};
use std::fs::read_to_string;

fn criterion_benchmark(c: &mut Criterion) {
    // from https://github.com/serde-rs/json-benchmark/blob/master/data/citm_catalog.json
    let citm_catalog_json = read_to_string("benches/json/citm_catalog.json").unwrap();
    let citm_catalog_json_large = create_json_array_of_object(&citm_catalog_json, 6);

    let tsconfig_json = read_to_string("benches/json/tsconfig.json").unwrap();
    let package_json = read_to_string("benches/json/package.json").unwrap();

    bench_function(c, "citm_catalog.json", &citm_catalog_json);
    bench_function(c, "citm_catalog.json large", &citm_catalog_json_large);
    bench_function(c, "tsconfig.json", &tsconfig_json);
    bench_function(c, "package.json", &package_json);

    fn bench_function(c: &mut Criterion, description: &str, json_text: &str) {
        c.bench_function(&format!("{} to ast", description), |b| {
            b.iter(|| {
                parse_to_ast(&json_text, &ParseOptions::default()).unwrap();
            })
        });

        c.bench_function(&format!("{} to value", description), |b| {
            b.iter(|| {
                parse_to_value(&json_text).unwrap();
            })
        });
    }

    fn create_json_array_of_object(text: &str, length: usize) -> String {
        let mut result = String::new();
        result.push_str("[");
        for i in 0..length {
            if i > 0 {
                result.push_str(",");
            }
            result.push_str(text);
        }
        result.push_str("]");
        result
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
