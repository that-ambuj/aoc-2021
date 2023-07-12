use aoc_2021::day9::{part1_solution, part2_solution, process_input};
use criterion::{black_box, criterion_group, criterion_main, Criterion, SamplingMode};

fn day9_bench(c: &mut Criterion) {
    let input = include_str!("../inputs/day9.txt");

    let puzzle = process_input(input);

    let mut group = c.benchmark_group("Day 9");
    group.sampling_mode(SamplingMode::Flat).sample_size(500);

    group.bench_function("Part 1", |b| b.iter(|| part1_solution(black_box(&puzzle))));
    group.bench_function("Part 2", |b| b.iter(|| part2_solution(black_box(&puzzle))));

    group.finish();
}

criterion_group!(benches, day9_bench);
criterion_main!(benches);
