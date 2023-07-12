use anyhow::{bail, Result};
use std::{collections::HashMap, str::FromStr};

fn main() {
    let input = include_str!("../../inputs/day5.txt");

    let processed_data = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(" -> ").unwrap();

            let p1 = Point::from_str(left)?;
            let p2 = Point::from_str(right)?;

            Ok(Line::new(p1, p2))
        })
        .collect::<Result<Vec<_>>>()
        .unwrap();

    let mut part1_counts = HashMap::new();

    for line in processed_data.clone() {
        for point in line.points(false) {
            part1_counts.insert(point, part1_counts.get(&point).unwrap_or(&0) + 1);
        }
    }

    let part1_answer = part1_counts
        .iter()
        .fold(0, |acc, (_, &count)| acc + (count > 1) as u32);

    dbg!(part1_answer);

    let mut part2_counts = HashMap::new();

    for line in processed_data {
        for point in line.points(true) {
            part2_counts.insert(point, part2_counts.get(&point).unwrap_or(&0) + 1);
        }
    }

    let part2_answer = part2_counts
        .iter()
        .fold(0, |acc, (_, &count)| acc + (count > 1) as u32);

    dbg!(part2_answer);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Point(u32, u32);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Line(Point, Point);

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Point(x, y)
    }
}

impl Line {
    pub fn new(p1: Point, p2: Point) -> Self {
        // Make sure that the `0`(from) field of the tuple struct is less than
        // `1`(to) field of the tuple struct.
        let min = std::cmp::min(p1, p2);
        let max = std::cmp::max(p1, p2);

        Line(min, max)
    }

    pub fn split(&self) -> (Point, Point) {
        (self.0, self.1)
    }

    pub fn points(&self, diagonal: bool) -> Vec<Point> {
        let (a, b) = self.split();

        if a.0 != b.0 && a.1 != b.1 {
            if !diagonal {
                return Vec::new();
            }

            steps(a.0, b.0)
                .zip(steps(a.1, b.1))
                .map(|(x, y)| Point::new(x, y))
                .collect()
        } else {
            steps(a.0, b.0)
                .flat_map(|x| steps(a.1, b.1).map(move |y| Point::new(x, y)))
                .collect()
        }
    }
}

fn steps(start: u32, end: u32) -> Box<dyn Iterator<Item = u32>> {
    if start < end {
        Box::new(start..=end)
    } else {
        Box::new((end..=start).rev())
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;

    /// Expects the provided string slice to be of format `x,y`.
    /// which can be split at `,` and parsed into Point struct
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((x, y)) = s.split_once(',') {
            let x = x.parse()?;
            let y = y.parse()?;

            Ok(Point::new(x, y))
        } else {
            bail!("Provided string is missing delimiter `,`.")
        }
    }
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (left, right) = s.split_once(" -> ").unwrap();

        let from = Point::from_str(left)?;
        let to = Point::from_str(right)?;

        Ok(Line(from, to))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_points_of_a_line() {
        let line = Line::new(Point(1, 1), Point(3, 3));

        let points = line.points(false);

        assert_eq!(points.len(), 3);
    }
}
