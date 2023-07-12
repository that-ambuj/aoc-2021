use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../../inputs/day7.txt");
    let (&input_line, _) = input.lines().collect::<Vec<_>>().split_first().unwrap();

    let crab_positions = parse_input(input_line);

    let part1 = solution(&crab_positions, false);
    let part2 = solution(&crab_positions, true);

    dbg!(part1);
    dbg!(part2);
}

/// Takes `input` as a single line representing the puzzle input
fn parse_input(input: &str) -> Vec<u32> {
    input
        .trim()
        .split(',')
        .map(|p| p.parse().unwrap())
        .collect()
}

fn solution(crab_positions: &[u32], exponential_fuel_cost: bool) -> f32 {
    let highest_pos = *crab_positions.iter().max().unwrap();
    let lowest_pos = *crab_positions.iter().min().unwrap();

    let mut crabs = HashMap::new();

    crab_positions.iter().for_each(|&pos| {
        crabs.insert(pos, crabs.get(&pos).unwrap_or(&0) + 1);
    });

    let total_crabs = crab_positions.len();
    // Find the point to move to, where the fuel cost is lowest
    let lowest_cost_avg = (lowest_pos..=highest_pos)
        // Find average cost for every crab to move to the current point
        .map(|curr_pos| {
            // Sum up fuel for each crab's journey
            let sum: u32 = crabs
                .iter()
                .map(|(&pos, count)| {
                    count * fuel_cost(distance(curr_pos, pos), exponential_fuel_cost)
                })
                .sum();

            sum as f32 / total_crabs as f32
        })
        // Take the lowest average fuel cost
        .min_by(|a1, a2| a1.total_cmp(a2))
        .unwrap();

    (lowest_cost_avg * total_crabs as f32).round()
}

/// Calculates distance between two points
///
/// In this problem, distance of a crab to a certain point
fn distance(p1: u32, p2: u32) -> u32 {
    match p1.cmp(&p2) {
        Ordering::Less => p2 - p1,
        Ordering::Greater => p1 - p2,
        Ordering::Equal => 0,
    }
}

/// Calculates the fuel cost for a given distance
///
/// `exponential` dictates the method for calculation of cost
fn fuel_cost(dist: u32, exponential: bool) -> u32 {
    if exponential {
        // Give the sum of 1 + 2 + 3 ..... + (n - 2) + (n - 1) + n
        // which is the formula n * (n + 1) / 2
        //
        // Hint: Lookup "sum of first n natural numbers"
        dist * (dist + 1) / 2
    } else {
        // For part 1, we don't need to calculate exponential fuel const
        // as it is already linear, we return the same distance
        dist
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn part1_example() {
        let positions = parse_input(EXAMPLE_INPUT);

        let answer = solution(&positions, false);

        assert!(answer.eq(&37.0));
    }

    #[test]
    fn part2_example() {
        let positions = parse_input(EXAMPLE_INPUT);

        let answer = solution(&positions, true);

        assert!(answer.eq(&168.0));
    }
}
