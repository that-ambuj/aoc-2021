fn main() {
    let input = include_str!("../../inputs/day3.txt");

    let lines = input.lines().collect::<Vec<&str>>();
    let half_length = lines.len() / 2;
    let width = lines[0].len();

    let mut sums = vec![0; width];

    lines.iter().for_each(|line| {
        let bits: Vec<_> = line.chars().filter_map(|c| c.to_digit(2)).collect();

        for i in 0..width {
            sums[i] += bits[i];
        }
    });

    // Binary representation of gamma as a String
    let gamma_bin = sums
        .into_iter()
        .map(|sum| if sum as usize > half_length { '1' } else { '0' })
        .collect::<String>();

    // Create number filled with 1 bits upto the width
    // of every given binary reading
    let mut xor_from: u16 = 0;

    for _ in 0..width {
        xor_from = (xor_from << 1) + 1;
    }

    // Convert String to u16
    let gamma = u16::from_str_radix(&gamma_bin, 2).unwrap();
    let epsilon = gamma ^ xor_from;

    let result = gamma as u64 * epsilon as u64;

    dbg!(result);
}
