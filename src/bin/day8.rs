use aoc_2021::day8::*;

fn main() {
    let puzzle_input = include_str!("../../inputs/day8.txt");

    let data = parse_input(puzzle_input);
    let (_, output_values): (Vec<_>, Vec<_>) = data.iter().cloned().unzip();

    let part1 = part1_solution(&output_values);
    let part2 = part2_solution(&data);

    dbg!(part1);
    dbg!(part2);
}
