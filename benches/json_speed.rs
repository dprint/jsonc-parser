use criterion::{criterion_group, criterion_main, Criterion};
use jsonc_parser::{parse_to_ast, parse_to_value, ParseOptions};
use std::fs::read_to_string;

fn criterion_benchmark(c: &mut Criterion) {
    let big_3m_json = read_to_string("benches/json/3M.json").unwrap();
    let tsconfig_json = read_to_string("benches/json/tsconfig.json").unwrap();
    let package_json = read_to_string("benches/json/package.json").unwrap();
    c.bench_function("3M json to ast", |b| {
        b.iter(|| {
            parse_to_value(&big_3m_json).unwrap();
        })
    });

    c.bench_function("3M json to value", |b| {
        b.iter(|| {
            parse_to_ast(&big_3m_json, &ParseOptions::default()).unwrap();
        })
    });

    c.bench_function("tsconfig.json to value", |b| {
        b.iter(|| {
            parse_to_value(&tsconfig_json).unwrap();
        })
    });

    c.bench_function("tsconfig.json to ast", |b| {
        b.iter(|| {
            parse_to_ast(&tsconfig_json, &ParseOptions::default()).unwrap();
        })
    });

    c.bench_function("package.json of vscode repository to value", |b| {
        b.iter(|| {
            parse_to_value(&package_json).unwrap();
        })
    });

    c.bench_function("package.json of vscode repository to ast", |b| {
        b.iter(|| {
            parse_to_ast(&package_json, &ParseOptions::default()).unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
