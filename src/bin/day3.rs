fn main() {
    let input = include_str!("../../inputs/day3.txt");

    let values: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(2)).collect())
        .collect();

    let o2_values = filter_values(values.clone(), |digit, most_common| digit == most_common);
    let co2_values = filter_values(values, |digit, most_common| digit != most_common);

    let o2_value = o2_values[0].iter().fold(0, |acc, val| (acc << 1) + val);
    let co2_value = co2_values[0].iter().fold(0, |acc, val| (acc << 1) + val);

    dbg!(o2_value * co2_value);
}

fn filter_values(values: Vec<Vec<u32>>, filter_fn: fn(u32, u32) -> bool) -> Vec<Vec<u32>> {
    let width = values[0].len();
    let half = values.len() / 2;

    let mut gamma: u16 = 0;
    let mut filtered_values = values.clone();

    for col in 0..width {
        let column_values: Vec<u32> = values
            .iter()
            .map(|binary_string| binary_string[col])
            .collect();

        // Sum is alias for occurences of `1` in the column
        let sum = column_values.iter().sum::<u32>();
        // Is `1` if there are more occurences of `1` than `0`
        let most_common = (sum as usize > half) as u32;

        if filtered_values.len() != 1 {
            filtered_values = filtered_values
                .into_iter()
                .filter(|value| filter_fn(value[col], most_common))
                .collect();
        }

        gamma = (gamma << 1) + most_common as u16
    }

    // 16 is the size of integer for gamma
    let padding_left = 16 - width;

    // This does a bitwise `NOT` on gamma and replaces the left `1`s with `0`s
    // because the input may be less than 16 bits(in this case, 12 bits)
    // and expects the answer to respect the number of bits
    //
    // This is acheived by shifting `padding`(in this case 4) `1` bytes
    // to the left and then to the right to insert `0`s in place of `1`s
    let epsilon = !gamma << padding_left >> padding_left;

    let part1_answer = gamma as u32 * epsilon as u32;
    dbg!(part1_answer);

    filtered_values
}
