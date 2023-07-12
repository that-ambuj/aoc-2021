use std::time::Duration;

use aoc_2021::day9::{part1_solution, part2_solution, process_input};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn day9_part1_bench(c: &mut Criterion) {
    let input = include_str!("../inputs/day9.txt");

    let puzzle = process_input(input);

    c.bench_function("day9 bench part-1", |b| {
        b.iter(|| part1_solution(black_box(&puzzle)))
    });
}

fn day9_part2_bench(c: &mut Criterion) {
    let input = include_str!("../inputs/day9.txt");

    let puzzle = process_input(input);

    c.bench_function("day9 bench part-2", |b| {
        b.iter(|| part2_solution(black_box(&puzzle)))
    });
}

criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = day9_part1_bench, day9_part2_bench
}

criterion_main!(benches);
