use std::collections::HashMap;

use itertools::Itertools;

pub fn part1_solution(input: &str) -> u32 {
    let score_ref = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    let score = input.lines().map(str::trim).fold(0, |acc, line| {
        let mut stack = vec![];

        for c in line.chars() {
            if is_left(c) {
                stack.push(c);
            } else {
                let last = stack.last().unwrap();
                let complement = left_part(c);

                if *last == complement {
                    stack.pop();
                } else {
                    return acc + score_ref.get(&c).unwrap();
                }
            }
        }

        acc
    });

    score
}

pub fn part2_solution(input: &str) -> u64 {
    let scores = input
        .lines()
        .filter_map(|line| {
            let mut stack = vec![];

            for c in line.chars() {
                if is_left(c) {
                    stack.push(c)
                } else {
                    let last = stack.last().unwrap();
                    let complement = left_part(c);

                    if *last == complement {
                        stack.pop();
                    } else {
                        // Skip Corrupted Lines
                        return None;
                    }
                }
            }

            if stack.len() > 0 {
                Some(stack.into_iter().rev().fold(0u64, |acc, c| {
                    acc * 5
                        + match c {
                            '(' => 1,
                            '[' => 2,
                            '{' => 3,
                            '<' => 4,
                            _ => panic!("I trusted thy input"),
                        }
                }))
            } else {
                panic!("Trust me bro")
            }
        })
        .sorted()
        .collect::<Vec<_>>();

    scores[scores.len() / 2]
}

fn is_left(c: char) -> bool {
    matches!(c, '(' | '[' | '{' | '<')
}

fn left_part(right: char) -> char {
    match right {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;

    #[test]
    fn day10_part1_example() {
        let score = part1_solution(EXAMPLE_INPUT);

        assert_eq!(score, 26397);
    }

    #[test]
    fn day10_part2_example() {
        let score = part2_solution(EXAMPLE_INPUT);

        assert_eq!(score, 288957);
    }
}
