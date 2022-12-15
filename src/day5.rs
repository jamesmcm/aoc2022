use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> (Vec<Vec<char>>, Vec<(i32, i32, i32)>) {
    todo!("Parse input here")
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &(Vec<Vec<char>>, Vec<(i32, i32, i32)>)) -> String {
    todo!("Do transformations here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let inp = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let parsed_input = input_generator(inp);

        assert_eq!(solve_part1(&parsed_input), "CMZ".to_owned());
    }
}
