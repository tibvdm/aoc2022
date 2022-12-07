use criterion::{criterion_group, Criterion};

use aoc2022::day05;

fn bench_part1(c: &mut Criterion) {
    c.bench_function("day05::part1", |b| b.iter(|| day05::part1()));
}

fn bench_part2(c: &mut Criterion) {
    c.bench_function("day05::part2", |b| b.iter(|| day05::part2()));
}

criterion_group!(day05, bench_part1, bench_part2);
