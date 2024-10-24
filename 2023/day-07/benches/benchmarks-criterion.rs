use core::time::Duration;
use criterion::{criterion_group, criterion_main, Criterion};
use day_07::*;

fn criterion_benchmark_part1(c: &mut Criterion) {
    let input = aoc_file_reader::read_file(FILENAME_PART_1);

    let mut group = c.benchmark_group(DAY_STR.to_owned() + "::part1");

    group.warm_up_time(Duration::from_millis(WARM_UP_TIME_MS));
    group.measurement_time(Duration::from_millis(MEASUREMENT_TIME_MS));

    group.bench_with_input("part1_enum", &input, |b, input| {
        b.iter(|| part1_enum::solve_puzzle(input))
    });
    group.bench_with_input("part1_fast", &input, |b, input| {
        b.iter(|| part1_fast::solve_puzzle(input))
    });
    group.bench_with_input("part1_parallel_sort", &input, |b, input| {
        b.iter(|| part1_parallel_sort::solve_puzzle(input))
    });
    group.finish();
}

fn criterion_benchmark_part2(c: &mut Criterion) {
    let input = aoc_file_reader::read_file(FILENAME_PART_2);

    let mut group = c.benchmark_group(DAY_STR.to_owned() + "::part2");

    group.warm_up_time(Duration::from_millis(WARM_UP_TIME_MS));
    group.measurement_time(Duration::from_millis(MEASUREMENT_TIME_MS));

    group.bench_with_input("part2_enum", &input, |b, input| {
        b.iter(|| part2_enum::solve_puzzle(input))
    });
    group.bench_with_input("part2_fast", &input, |b, input| {
        b.iter(|| part2_fast::solve_puzzle(input))
    });
    group.finish();
}

criterion_group!(
    benches,
    criterion_benchmark_part1,
    criterion_benchmark_part2
);
criterion_main!(benches);