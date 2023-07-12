fn main() {
    let input = include_str!("../../inputs/day3.txt");

    let values: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(2).unwrap()).collect())
        .collect();

    let width = values[0].len();
    let size = 16;

    let gamma = part1(&values);

    let padding = size - width;
    let epsilon = !gamma << padding >> padding;

    let power_consumption = gamma as usize * epsilon as usize;

    dbg!(power_consumption);

    let o2_value = part2(&values, |digit, most_common| digit == most_common);
    let co2_value = part2(&values, |digit, most_common| digit != most_common);

    let life_support_rating = o2_value * co2_value;
    dbg!(life_support_rating);
}

fn vec_to_int(slice: &[u32]) -> u32 {
    slice.iter().fold(0, |acc, val| (acc << 1) + val)
}

fn part1(values: &[Vec<u32>]) -> u16 {
    let width = values[0].len();
    let half = values.len() / 2;

    let mut gamma = 0u16;

    for col in 0..width {
        let column_values: Vec<u32> = values
            .iter()
            .map(|binary_string| binary_string[col])
            .collect();

        // Sum is alias for occurences of `1` in the column
        let sum = column_values.iter().sum::<u32>();
        // Is `1` if there are more occurences of `1` than `0`
        let most_common = (sum as usize > half) as u16;

        gamma = (gamma << 1) + most_common;
    }

    gamma
}

fn part2(values: &[Vec<u32>], filter_fn: fn(u32, u32) -> bool) -> u32 {
    let width = values[0].len();

    let mut values = values.to_owned();

    for col in 0..width {
        let column_values: Vec<u32> = values.iter().map(|digits| digits[col]).collect();

        let half = (values.len() + 1) / 2;
        // Sum is alias for occurences of `1` in the column
        let sum = column_values.iter().sum::<u32>();
        // Is `1` if there are more occurences of `1` than `0`
        let most_common = (sum as usize >= half) as u32;

        values.retain(|value| filter_fn(value[col], most_common));

        if values.len() == 1 {
            break;
        }
    }

    vec_to_int(&values[0])
}
