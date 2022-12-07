use criterion::{criterion_group, Criterion};

use aoc2022::day04;

fn bench_part1(c: &mut Criterion) {
    c.bench_function("day04::part1", |b| b.iter(|| day04::part1()));
}

fn bench_part2(c: &mut Criterion) {
    c.bench_function("day04::part2", |b| b.iter(|| day04::part2()));
}

criterion_group!(day04, bench_part1, bench_part2);
