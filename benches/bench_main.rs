use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benchmarks::day01::day01,
    benchmarks::day02::day02,
    benchmarks::day03::day03,
    benchmarks::day04::day04,
    benchmarks::day05::day05,
    benchmarks::day06::day06
}
