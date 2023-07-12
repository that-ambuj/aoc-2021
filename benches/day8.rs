use aoc_2021::day8::{parse_input, part1_solution, part2_solution};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn day8_part1_bench(c: &mut Criterion) {
    let puzzle_input = include_str!("../inputs/day8.txt");

    let data = parse_input(puzzle_input);
    let (_, output_values): (Vec<_>, Vec<_>) = data.iter().cloned().unzip();

    c.bench_function("day8 bench part1", |b| {
        b.iter(|| part1_solution(black_box(&output_values)))
    });
}

fn day8_part2_bench(c: &mut Criterion) {
    let puzzle_input = include_str!("../inputs/day8.txt");

    let data = parse_input(puzzle_input);

    c.bench_function("day8 bench part2", |b| {
        b.iter(|| part2_solution(black_box(&data)))
    });
}

criterion_group!(benches, day8_part1_bench, day8_part2_bench);
criterion_main!(benches);
