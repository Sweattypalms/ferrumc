#![allow(unused)]

use ferrumc_utils::utils::{MinecraftReaderExt, MinecraftWriterExt};
use std::io::Cursor;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn varint_benches(c: &mut Criterion) {
    let mut data = Cursor::new([0u8; 16]);
    c.bench_function("varint write", |b| {
        b.iter(|| data.write_varint(black_box(6)))
    });
    c.bench_function("varint read", |b| b.iter(|| data.read_varint()));
}

pub fn varstring_benches(c: &mut Criterion) {
    let mut data = Cursor::new(vec![]);
    c.bench_function("varstring write", |b| {
        b.iter(|| data.write_varstring(black_box("Hello World")))
    });
    c.bench_function("varstring read", |b| b.iter(|| data.read_varstring()));
}

criterion_group!(benches, varint_benches, varstring_benches);
criterion_main!(benches);