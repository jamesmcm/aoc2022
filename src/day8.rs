use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|x| x.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect()
}

pub fn check_heights(input: &[Vec<i32>], row: usize, col: usize) -> bool {
    let left: bool = (0..col).map(|c| input[row][c] < input[row][col]).all(|x| x);
    let right: bool = (col + 1..input[0].len())
        .map(|c| input[row][c] < input[row][col])
        .all(|x| x);

    let up: bool = (0..row).map(|r| input[r][col] < input[row][col]).all(|x| x);
    let down: bool = (row + 1..input.len())
        .map(|r| input[r][col] < input[row][col])
        .all(|x| x);

    left || right || up || down
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[Vec<i32>]) -> i32 {
    let height = input.len();
    let width = input[0].len();

    let borders = ((2 * height) as i32 + (2 * width) as i32) - 4;

    let mut inner: i32 = 0;

    for row in 1..(height - 1) {
        for col in 1..(width - 1) {
            if check_heights(input, row, col) {
                inner += 1;
            }
        }
    }

    borders + inner
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let inp = "30373
25512
65332
33549
35390";

        let parsed_input = input_generator(inp);

        assert_eq!(solve_part1(&parsed_input), 21);
    }
}
