use criterion::{criterion_group, Criterion};

use aoc2022::day02;

fn bench_part1(c: &mut Criterion) {
    c.bench_function("day02::part1", |b| b.iter(|| day02::part1()));
}

fn bench_part2(c: &mut Criterion) {
    c.bench_function("day02::part2", |b| b.iter(|| day02::part2()));
}

criterion_group!(day02, bench_part1, bench_part2);
