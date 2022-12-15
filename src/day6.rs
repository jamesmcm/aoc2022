use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> String {
    input.to_owned()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut c: i32 = 0;
    for win in input.chars().collect::<Vec<char>>().as_slice().windows(4) {
        if HashSet::<char>::from_iter(win.iter().copied()).len() == 4 {
            break;
        }
        c += 1;
    }
    c + 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        let inp = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let parsed_input = input_generator(inp);
        assert_eq!(solve_part1(&parsed_input), 7);
    }
    #[test]
    fn test_part1_2() {
        let inp = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let parsed_input = input_generator(inp);
        assert_eq!(solve_part1(&parsed_input), 5);
    }
    #[test]
    fn test_part1_3() {
        let inp = "nppdvjthqldpwncqszvftbrmjlhg";
        let parsed_input = input_generator(inp);
        assert_eq!(solve_part1(&parsed_input), 6);
    }
    #[test]
    fn test_part1_4() {
        let inp = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let parsed_input = input_generator(inp);
        assert_eq!(solve_part1(&parsed_input), 10);
    }
    #[test]
    fn test_part1_5() {
        let inp = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let parsed_input = input_generator(inp);
        assert_eq!(solve_part1(&parsed_input), 11);
    }
}
