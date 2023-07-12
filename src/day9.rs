use itertools::Itertools;
use std::collections::HashSet;

pub const EXAMPLE_INPUT: &str = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;

pub fn process_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(str::trim)
        .map(|str| str.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

pub fn part1_solution(puzzle: &[Vec<u32>]) -> u32 {
    let low_points = find_low_points(puzzle);

    low_points.iter().map(|(x, y)| puzzle[*x][*y] + 1).sum()
}

fn find_low_points(rows: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let max_length = rows[0].len();

    rows.iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter().enumerate().filter_map(move |(col_idx, col)| {
                let mut is_low = true;

                if col_idx > 0 {
                    let left = row[col_idx - 1];
                    is_low &= *col < left;
                }

                if col_idx < max_length - 1 {
                    let right = row[col_idx + 1];
                    is_low &= *col < right;
                }

                if row_idx > 0 {
                    let above = rows[row_idx - 1][col_idx];
                    is_low &= *col < above;
                }

                if row_idx < rows.len() - 1 {
                    let below = rows[row_idx + 1][col_idx];
                    is_low &= *col < below;
                }

                if is_low {
                    Some((row_idx, col_idx))
                } else {
                    None
                }
            })
        })
        .collect()
}

pub fn part2_solution(puzzle: &[Vec<u32>]) -> u32 {
    let row_size = puzzle.len();
    let col_size = puzzle[0].len();

    let anchors = find_low_points(&puzzle);

    let mut basins = Vec::new();
    let mut counted = HashSet::new();

    for (anchor_x, anchor_y) in anchors.into_iter() {
        counted.insert((anchor_x, anchor_y));

        let anchor_val = puzzle[anchor_x][anchor_y];

        let mut to_visit = cardinal_directions(anchor_x, anchor_y, row_size, col_size)
            .into_iter()
            .map(|(a, b)| (a, b, anchor_val))
            .collect_vec();

        let mut current_basin_size = 1;

        while let Some((x, y, comparison_val)) = to_visit.pop() {
            let visitor = (x, y);

            if counted.contains(&visitor) {
                continue;
            }

            let val = puzzle[x][y];

            if val > comparison_val && val != 9 {
                current_basin_size += 1;

                to_visit.extend(
                    cardinal_directions(x, y, row_size, col_size)
                        .into_iter()
                        .map(|(a, b)| (a, b, val)),
                );

                counted.insert(visitor);
            }
        }

        basins.push(current_basin_size);
    }

    basins.iter().sorted().rev().take(3).product()
}

// Gives locations w.r.t. current point that can be safely accessed
fn cardinal_directions(x: usize, y: usize, x_bound: usize, y_bound: usize) -> Vec<(usize, usize)> {
    let mut directions = Vec::new();

    if let Some(x) = x.checked_sub(1) {
        directions.push((x, y));
    }

    if let Some(y) = y.checked_sub(1) {
        directions.push((x, y));
    }

    if let Some(x) = x.checked_add(1) {
        if x < x_bound {
            directions.push((x, y))
        }
    }

    if let Some(y) = y.checked_add(1) {
        if y < y_bound {
            directions.push((x, y))
        }
    }

    directions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9_input_parsing() {
        let input = "123\n456\n789";
        let data = process_input(input);

        assert_eq!(data.as_slice(), [[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    }

    #[test]
    fn day9_part1_example() {
        let data = process_input(EXAMPLE_INPUT);

        assert_eq!(part1_solution(&data), 15)
    }

    #[test]
    fn day9_part2_example() {
        let data = process_input(EXAMPLE_INPUT);

        assert_eq!(part2_solution(&data), 1134)
    }
}
