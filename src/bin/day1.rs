use itertools::Itertools;

fn main() {
    let input = include_str!("../../inputs/day1.txt");
    // Parse each line into a number reading of it's own
    let lines_iter = input.lines().map(|x| x.parse::<u32>().unwrap());

    let result = lines_iter
        .tuple_windows()
        // Take measurements window and sum them up
        .map(|(x, y, z)| x + y + z)
        .tuple_windows()
        // Compare the sums in pairs and count the number of increments
        .fold(0 as u32, |inc_count, (before, after)| {
            if after > before {
                return inc_count + 1;
            }

            inc_count
        });

    println!(
        "There are {} sums that are larger than the previous sum.",
        result
    );
}
