use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashSet;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|l| (l[..l.len() / 2].to_owned(), l[l.len() / 2..].to_owned()))
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[(String, String)]) -> i32 {
    input
        .into_iter()
        .map(|inp| -> i32 {
            let set1: HashSet<char> = HashSet::from_iter(inp.0.chars());
            let set2: HashSet<char> = HashSet::from_iter(inp.1.chars());

            let common = set1.intersection(&set2);

            common.into_iter().map(|c| char_to_points(*c)).sum()
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[(String, String)]) -> i32 {
    input
        .chunks(3)
        .map(|chunk| -> i32 {
            let finalset: HashSet<char> = chunk
                .into_iter()
                .map(|l| HashSet::from_iter((l.0.clone() + &l.1).chars()))
                .reduce(|x, y| HashSet::from_iter(x.intersection(&y).map(|c| *c)))
                .unwrap();
            finalset.into_iter().map(|c| char_to_points(c)).sum()
        })
        .sum()
}

pub fn char_to_points(c: char) -> i32 {
    let ascii = (c as u32) as i32;

    if ascii >= 97 {
        ascii - 96
    } else {
        ascii - 38
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let inp = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let parsed_input = input_generator(inp);

        assert_eq!(solve_part1(&parsed_input), 157);
    }

    #[test]
    fn test_2() {
        let inp = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let parsed_input = input_generator(inp);

        assert_eq!(solve_part2(&parsed_input), 70);
    }
}
