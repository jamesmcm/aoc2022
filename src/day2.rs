use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn new(c: &str) -> anyhow::Result<Self> {
        use Move::*;

        match c {
            "A" => Ok(Rock),
            "B" => Ok(Paper),
            "C" => Ok(Scissors),
            "X" => Ok(Rock),
            "Y" => Ok(Paper),
            "Z" => Ok(Scissors),
            _ => Err(anyhow::anyhow!("Bad char for Move")),
        }
    }

    pub fn win_move(&self) -> Self {
        use Move::*;
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    pub fn lose_move(&self) -> Self {
        use Move::*;
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    pub fn points(&self) -> i32 {
        use Move::*;
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

pub fn win_points(pair: &(Move, Move)) -> i32 {
    use Move::*;
    match pair {
        (Rock, Rock) => 3,
        (Rock, Paper) => 6,
        (Rock, Scissors) => 0,
        (Paper, Rock) => 0,
        (Paper, Paper) => 3,
        (Paper, Scissors) => 6,
        (Scissors, Rock) => 6,
        (Scissors, Scissors) => 3,
        (Scissors, Paper) => 0,
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(Move, Move)> {
    input
        .lines()
        .map(|l| {
            let v: Vec<Move> = l
                .split_whitespace()
                .map(|c| Move::new(c).unwrap())
                .collect();

            (v[0], v[1])
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[(Move, Move)]) -> i32 {
    input
        .into_iter()
        .map(|m| m.1.points() + win_points(&m))
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[(Move, Move)]) -> i32 {
    use Move::*;
    input
        .into_iter()
        .map(|m| {
            let m2 = match m.1 {
                Rock => (0, m.0.lose_move()),
                Paper => (3, m.0),
                Scissors => (6, m.0.win_move()),
            };

            m2.1.points() + m2.0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let inp = "A Y
B X
C Z";

        let parsed_input = input_generator(inp);

        assert_eq!(solve_part1(&parsed_input), 15);
    }

    #[test]
    fn test_2() {
        let inp = "A Y
B X
C Z";

        let parsed_input = input_generator(inp);

        assert_eq!(solve_part2(&parsed_input), 12);
    }
}
