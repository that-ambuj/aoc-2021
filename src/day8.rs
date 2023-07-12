use std::collections::HashSet;

use rayon::{
    prelude::{IntoParallelRefIterator, ParallelIterator},
    str::ParallelString,
};

pub fn parse_input(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| line.split_once(" | ").unwrap())
        .collect::<Vec<_>>()
}

pub fn part1_solution(output_values: &[&str]) -> usize {
    output_values
        .par_iter()
        .flat_map(|line| line.par_split(' '))
        .filter(|&value| matches!(value.len(), 2 | 3 | 4 | 7))
        .count()
}

pub fn part2_solution(input: &[(&str, &str)]) -> u32 {
    let answer = input
        .par_iter()
        .map(|(signal, output)| {
            let mut values = parse_signal(signal);

            let required_values = parse_signal(output);

            // Only keep the values to decode that are useful in decoding other numbers
            // or are part of the four digit display
            values.retain(|val| required_values.contains(val) || matches!(val.len(), 2 | 4));

            let decoded_signal = decode_signal(&values);

            required_values
                .into_iter()
                .map(|v| decoded_signal.iter().find(|(set, _)| **set == v).unwrap())
                .map(|(_, number)| char::from_digit(*number as u32, 10).unwrap())
                .collect::<String>()
                .parse::<u32>()
                .unwrap()
        })
        .sum();

    answer
}

fn parse_signal(input: &str) -> Vec<HashSet<char>> {
    input
        .par_split(' ')
        .map(|str| str.chars().collect::<HashSet<_>>())
        .collect()
}

pub fn decode_signal(values: &[HashSet<char>]) -> Vec<(&HashSet<char>, u8)> {
    let mut decoded = values
        .par_iter()
        .filter_map(|input| match input.len() {
            // Just a type hint for rust to infer it as u8 instead of i32
            2 => Some((input, 1u8)),
            4 => Some((input, 4)),
            3 => Some((input, 7)),
            7 => Some((input, 8)),
            _ => None,
        })
        .collect::<Vec<_>>();

    let number_1 = decoded.iter().find(|(_, n)| *n == 1).unwrap().0;
    let number_4 = decoded.iter().find(|(_, n)| *n == 4).unwrap().0;

    values.iter().for_each(|input| {
        match input.len() {
            2 | 3 | 4 | 7 => {}
            5 => {
                match (
                    &input.intersection(number_1).count(),
                    &input.intersection(number_4).count(),
                ) {
                    (2, _) => decoded.push((input, 3)),
                    (_, 2) => decoded.push((input, 2)),
                    _ => decoded.push((input, 5)),
                }
            }
            6 => {
                match (
                    &input.intersection(number_1).count(),
                    &input.intersection(number_4).count(),
                ) {
                    (1, _) => decoded.push((input, 6)),
                    (_, 4) => decoded.push((input, 9)),
                    _ => decoded.push((input, 0)),
                }
            }
            _ => panic!(),
        };
    });

    decoded
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
"#;

    #[test]
    fn day8_part1_example() {
        let data = parse_input(EXAMPLE_INPUT);

        let (_, output_values): (Vec<_>, Vec<_>) = data.iter().cloned().unzip();
        let answer = part1_solution(&output_values);

        assert_eq!(answer, 26)
    }

    #[test]
    fn day8_part2_example() {
        let data = parse_input(EXAMPLE_INPUT);

        let answer = part2_solution(&data);

        assert_eq!(answer, 61229);
    }
}
