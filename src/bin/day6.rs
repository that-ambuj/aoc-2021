use std::collections::VecDeque;

fn main() {
    let input = include_str!("../../inputs/day6.txt");
    let input_line = input.lines().take(1).collect::<String>();

    let puzzle = input_line
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<usize>>();

    let part1 = count_fish(&puzzle, 80);
    let part2 = count_fish(&puzzle, 256);

    dbg!(part1);
    dbg!(part2);
}

fn count_fish(puzzle: &[usize], days: u32) -> u128 {
    // Each index represents days-until-new-baby and the value at that index represents
    // the how many lanternfish have that timer(alias `index`)
    //
    // For visualization
    // index: [0, 1, 2, 3, 4, 5, 6, 7, 8]
    // value: [4, 5, 1, 3, 4, 5, 9, 8, 2]
    //         ^                 ^     ^ Newly born lanternfish
    //         |                 | Old fish that have baby every 7 days
    //         | Number of lanternfish that will have a baby today
    let mut counts = VecDeque::from(vec![0u128; 9]);

    puzzle
        .iter()
        .for_each(|&days_till_baby| counts[days_till_baby] += 1);

    (0..days).for_each(|_| {
        // Returns the number of fish that are at timer 0 (which had a baby today)
        let new_babies = counts.pop_front().unwrap();

        // Put Old fish at index 6 to reproduce
        counts[6] += new_babies;
        // Put new fish at the end to reproduce
        counts.push_back(new_babies);
    });

    counts.iter().sum::<u128>()
}
