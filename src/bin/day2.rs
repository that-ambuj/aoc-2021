fn main() {
    let input = include_str!("../../inputs/day2.txt");

    let mut horizontal_pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    // Process the given data
    let iter = input.lines().map(|line| {
        if let Some((instruction, steps)) = line.split_once(" ") {
            return (instruction, steps);
        } else {
            unreachable!("Each line is guaranteed to have an instruction and steps value.")
        }
    });

    for (instruction, steps) in iter {
        let steps = steps.parse::<u32>().unwrap();

        match instruction {
            "forward" => {
                horizontal_pos += steps;
                depth += aim * steps;
            }
            "up" => aim -= steps,
            "down" => aim += steps,
            _ => unreachable!("The input is guaranteed to match above cases."),
        }
    }

    let result = horizontal_pos * depth;

    dbg!(result);
}
