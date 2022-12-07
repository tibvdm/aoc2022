use criterion::{criterion_group, Criterion};

use aoc2022::day03;

fn bench_part1(c: &mut Criterion) {
    c.bench_function("day03::part1", |b| b.iter(|| day03::part1()));
}

fn bench_part2(c: &mut Criterion) {
    c.bench_function("day03::part2", |b| b.iter(|| day03::part2()));
}

criterion_group!(day03, bench_part1, bench_part2);
