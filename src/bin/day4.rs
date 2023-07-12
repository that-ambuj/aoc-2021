fn main() {
    let input = include_str!("../../inputs/day4.txt");

    let (boards, numbers_to_draw) = Board::new(input);

    let part1_score = part1(&numbers_to_draw.0, boards.clone());
    dbg!(part1_score);

    let part2_score = part2(&numbers_to_draw.0, boards);
    dbg!(part2_score);
}

fn part1(numbers_to_draw: &[u8], boards: Vec<Board<NotWinning>>) -> u32 {
    let mut part1_score = 0;

    let mut boards = boards;

    for number in numbers_to_draw.iter() {
        boards
            .iter_mut()
            .for_each(|board| board.mark_number(*number));

        let winning_board = boards.iter().find_map(|board| board.winning());

        if let Some(board) = winning_board {
            part1_score = board.calculate_sum() * *number as u32;
            break;
        }
    }

    part1_score
}

fn part2(numbers_to_draw: &[u8], boards: Vec<Board<NotWinning>>) -> u32 {
    let mut last_winning = boards;

    for number in numbers_to_draw {
        // Mark current number on the board
        last_winning
            .iter_mut()
            .for_each(|board| board.mark_number(*number));

        // Filter out the ones that already won
        last_winning = last_winning
            .into_iter()
            .filter(|board| board.winning().is_none())
            .collect::<Vec<_>>();

        if last_winning.len() == 1 {
            break;
        }
    }

    let mut last_winning = last_winning[0];

    let winning_number = numbers_to_draw.iter().find(|number| {
        last_winning.mark_number(**number);
        last_winning.winning().is_some()
    });

    last_winning.winning().unwrap().calculate_sum() * *winning_number.unwrap() as u32
}

/// Represents a 5x5 matrix as a list of 5 rows of 5 numbers inside
#[derive(Debug, Default, Copy, Clone)]
struct Board<T: BoardState = NotWinning>(pub [[BoardNumber; 5]; 5], std::marker::PhantomData<T>);

trait BoardState {}

impl BoardState for Winning {}
impl BoardState for NotWinning {}

#[derive(Debug, Default, Clone, Copy)]
struct Winning;
#[derive(Debug, Default, Clone, Copy)]
struct NotWinning;

/// Represents a number on the board and it's marked state
#[derive(Debug, Default, Copy, Clone)]
struct BoardNumber {
    num: u8,
    is_marked: bool,
}

/// Represents a vector of numbers to be drawn
#[derive(Debug, Clone)]
struct NumbersToDraw(Vec<u8>);

impl Board<NotWinning> {
    pub fn new(input: &str) -> (Vec<Board<NotWinning>>, NumbersToDraw) {
        let mut processed_input = input.split("\n\n").collect::<Vec<&str>>().into_iter();

        let numbers = processed_input
            .next()
            .unwrap()
            .split(',')
            .map(|num_str| num_str.parse().unwrap())
            .collect();

        let boards = processed_input
            .map(|matrix| {
                let rows: Vec<_> = matrix.lines().map(line_to_row_numbers).collect();

                rows
            })
            .map(|rows| {
                let mut board = Board::default();
                board.0.copy_from_slice(&rows[0..5]);

                board
            })
            .collect::<Vec<_>>();

        (boards, NumbersToDraw(numbers))
    }

    pub fn mark_number(&mut self, number_to_mark: u8) {
        for row in 0..5 {
            for num in 0..5 {
                if self.0[row][num].num == number_to_mark {
                    self.0[row][num].is_marked = true;
                }
            }
        }
    }

    pub fn winning(&self) -> Option<Board<Winning>> {
        let is_any_row_marked = self.0.iter().any(|row| row.iter().all(|num| num.is_marked));

        let mut columns: Vec<Vec<BoardNumber>> = vec![];

        for col in 0..5 {
            columns.push(self.0.iter().map(|row| row[col]).collect());
        }

        let is_any_col_marked = columns
            .iter()
            .any(|col| col.iter().all(|num| num.is_marked));

        if is_any_row_marked || is_any_col_marked {
            Some(Board(self.0, std::marker::PhantomData))
        } else {
            None
        }
    }
}

impl Board<Winning> {
    pub fn calculate_sum(&self) -> u32 {
        self.0
            .map(|row| {
                row.iter()
                    .filter(|b| !b.is_marked)
                    .map(|b| b.num as u32)
                    .sum()
            })
            .iter()
            .sum()
    }
}

impl BoardNumber {
    fn new(num: u8) -> Self {
        BoardNumber {
            num,
            is_marked: false,
        }
    }
}

fn line_to_row_numbers(line: &str) -> [BoardNumber; 5] {
    let mut arr: [BoardNumber; 5] = Default::default();

    let numbers = line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .map(BoardNumber::new)
        .take(5)
        .collect::<Vec<_>>();

    arr.copy_from_slice(&numbers[..5]);

    arr
}
