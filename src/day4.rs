use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug)]
pub struct CleanRange {
    pub start: i32,
    pub end: i32,
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<(CleanRange, CleanRange)> {
    input
        .lines()
        .map(|l| {
            let pair = l
                .split(",")
                .map(|p| {
                    p.split("-")
                        .map(|n| n.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>();
            (
                CleanRange {
                    start: pair[0][0],
                    end: pair[0][1],
                },
                CleanRange {
                    start: pair[1][0],
                    end: pair[1][1],
                },
            )
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[(CleanRange, CleanRange)]) -> usize {
    input
        .into_iter()
        .filter(|l| {
            (l.0.start <= l.1.start && l.0.end >= l.1.end)
                || (l.1.start <= l.0.start && l.1.end >= l.0.end)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let inp = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        let parsed_input = input_generator(inp);

        assert_eq!(solve_part1(&parsed_input), 2);
    }
}
