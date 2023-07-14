use aoc_2021::day10::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn day10_bench(c: &mut Criterion) {
    let puzzle = include_str!("../inputs/day10.txt");

    let mut group = c.benchmark_group("Day 10");
    group.sample_size(250);

    group.bench_function("Part 1", |b| b.iter(|| part1_solution(black_box(&puzzle))));
    group.bench_function("Part 2", |b| b.iter(|| part2_solution(black_box(&puzzle))));

    group.finish();
}

criterion_group!(benches, day10_bench);
criterion_main!(benches);
