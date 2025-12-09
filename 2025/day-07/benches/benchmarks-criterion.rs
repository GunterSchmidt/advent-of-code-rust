use core::time::Duration;
use criterion::{criterion_group, criterion_main, Criterion};
use day_2025_07::*;

const DAY_STR: &str = "day_2025_07";
const WARM_UP_TIME_MS: u64 = 500;
const MEASUREMENT_TIME_MS: u64 = 2000;

fn criterion_benchmark(c: &mut Criterion) {
    let input = aoc_file_reader::read_file(FILENAME_PART_1);

    let mut group = c.benchmark_group(DAY_STR.to_owned());

    group.warm_up_time(Duration::from_millis(WARM_UP_TIME_MS));
    group.measurement_time(Duration::from_millis(MEASUREMENT_TIME_MS));

    group.bench_with_input("parse1 only", &input, |b, input| {
        b.iter(|| parse_data(input))
    });
    group.bench_with_input("part1 ascending", &input, |b, input| {
        b.iter(|| part1::solve_puzzle(input))
    });

    // Unclear why parsing alone is slower than part 2 which includes parsing.
    group.bench_with_input("part2", &input, |b, input| {
        b.iter(|| part2::solve_puzzle(input))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark,);
criterion_main!(benches);
