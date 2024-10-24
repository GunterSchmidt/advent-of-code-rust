use core::time::Duration;
use criterion::{criterion_group, criterion_main, Criterion};
use day_08::*;

fn criterion_benchmark_part1(c: &mut Criterion) {
    let input = aoc_file_reader::read_file(FILENAME_PART_1);

    let mut group = c.benchmark_group(DAY_STR.to_owned() + "::part1");

    group.warm_up_time(Duration::from_millis(WARM_UP_TIME_MS));
    group.measurement_time(Duration::from_millis(MEASUREMENT_TIME_MS));

    group.bench_with_input("part1_hashmap", &input, |b, input| {
        b.iter(|| part1_hashmap::solve_puzzle(input))
    });
    group.bench_with_input("part1_array", &input, |b, input| {
        b.iter(|| part1_array::solve_puzzle_array(input))
    });

    group.finish();
}

fn criterion_benchmark_part2(c: &mut Criterion) {
    let input = aoc_file_reader::read_file(FILENAME_PART_2);

    let mut group = c.benchmark_group(DAY_STR.to_owned() + "::part2");

    group.warm_up_time(Duration::from_millis(WARM_UP_TIME_MS));
    group.measurement_time(Duration::from_millis(MEASUREMENT_TIME_MS));

    // remove this benchmark and the same benchmark below will double in time
    group.bench_with_input("part2_array", &input, |b, input| {
        b.iter(|| part2_v3_array::solve_puzzle_array(input))
    });

    group.bench_with_input("part2_v1", &input, |b, input| {
        b.iter(|| part2_v1::solve_puzzle_v1(input))
    });
    group.bench_with_input("part2_v2", &input, |b, input| {
        b.iter(|| part2_v2::solve_puzzle_v2(input))
    });
    group.bench_with_input("part2_array_same", &input, |b, input| {
        b.iter(|| part2_v3_array::solve_puzzle_array(input))
    });
    group.bench_with_input("part2_v4_array_16_bit_key", &input, |b, input| {
        b.iter(|| part2_v4_array_16_bit_key::solve_puzzle_array_16_bit_key(input))
    });

    group.finish();
}

criterion_group!(
    benches,
    criterion_benchmark_part1,
    criterion_benchmark_part2,
);
criterion_main!(benches);
