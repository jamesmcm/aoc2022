use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
    let mut agg = input.lines().into_iter().fold(
        (Vec::new(), Vec::new()),
        |mut acc: (Vec<Vec<i32>>, Vec<i32>), l: &str| {
            if l == "" {
                acc.0.push(acc.1.clone());
                acc.1.clear();
                acc
            } else {
                acc.1.push(l.parse::<i32>().unwrap());
                acc
            }
        },
    );

    agg.0.push(agg.1);
    agg.0
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[Vec<i32>]) -> i32 {
    input.into_iter().map(|x| x.iter().sum()).max().unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[Vec<i32>]) -> i32 {
    let mut sums: Vec<i32> = input.into_iter().map(|x| x.iter().sum()).collect();
    sums.sort_by(|a, b| b.cmp(a));
    sums[0..3].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let inp = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        let parsed_input = input_generator(inp);

        assert_eq!(solve_part1(&parsed_input), 24000);
    }

    #[test]
    fn test_2() {
        let inp = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        let parsed_input = input_generator(inp);

        assert_eq!(solve_part2(&parsed_input), 45000);
    }
}
