use core::time::Duration;
use criterion::{criterion_group, criterion_main, Criterion};
use day_11::*;

fn criterion_benchmark_part1(c: &mut Criterion) {
    let input = aoc_file_reader::read_file(FILENAME_PART_1);

    let mut group = c.benchmark_group(DAY_STR.to_owned() + "::part1");

    group.warm_up_time(Duration::from_millis(WARM_UP_TIME_MS));
    group.measurement_time(Duration::from_millis(MEASUREMENT_TIME_MS));

    group.bench_with_input("part1 HashMap", &input, |b, input| {
        b.iter(|| puzzle::solve_part1_standalone(input))
    });
    group.bench_with_input("part1 simple", &input, |b, input| {
        b.iter(|| puzzle::solve_part1_create_all(crate::parse_data(input), 25))
    });

    group.finish();
}

fn criterion_benchmark_part2(c: &mut Criterion) {
    let input = aoc_file_reader::read_file(FILENAME_PART_2);

    let mut group = c.benchmark_group(DAY_STR.to_owned() + "::part2");

    group.warm_up_time(Duration::from_millis(WARM_UP_TIME_MS));
    group.measurement_time(Duration::from_millis(MEASUREMENT_TIME_MS));

    group.bench_with_input("part2", &input, |b, input| {
        b.iter(|| puzzle::solve_part2_standalone(input))
    });
    group.finish();
}

fn criterion_benchmark_both(c: &mut Criterion) {
    let input = aoc_file_reader::read_file(FILENAME_PART_2);

    let mut group = c.benchmark_group(DAY_STR.to_owned() + "::part2");

    group.warm_up_time(Duration::from_millis(WARM_UP_TIME_MS));
    group.measurement_time(Duration::from_millis(MEASUREMENT_TIME_MS));

    group.bench_with_input("together", &input, |b, input| {
        b.iter(|| puzzle::solve_both(input))
    });
    group.finish();
}

criterion_group!(
    benches,
    criterion_benchmark_part1,
    criterion_benchmark_part2,
    criterion_benchmark_both
);
criterion_main!(benches);
