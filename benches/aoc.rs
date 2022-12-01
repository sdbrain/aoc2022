use aoc2022::day1;
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = fs::read_to_string("./src/data/day1.txt").unwrap();
    let day1 = day1::Day1::new();
    c.bench_function("day1bv1", |b| b.iter(|| day1.top_3_elves_v1(&input)));
    c.bench_function("day1bv2", |b| b.iter(|| day1.top_3_elves_v2(&input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
