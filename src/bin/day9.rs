use aoc_2021::day9::*;

fn main() {
    let input = include_str!("../../inputs/day9.txt");
    let puzzle = process_input(input);

    let part1 = part1_solution(&puzzle);
    let part2 = part2_solution(&puzzle);

    dbg!(part1);
    dbg!(part2);
}
