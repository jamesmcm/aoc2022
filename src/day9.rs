use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Move {
    direction: Dir,
    magnitude: usize,
}

impl Move {
    pub fn from_s(s: &str) -> Self {
        let mut split = s.split(' ');
        let dir = Dir::from_s(split.next().unwrap());
        let magnitude: usize = split.next().unwrap().parse().unwrap();

        Self {
            direction: dir,
            magnitude,
        }
    }
}

#[derive(Debug)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn from_s(c: &str) -> Self {
        use Dir::*;
        match c {
            "R" => Right,
            "D" => Down,
            "U" => Up,
            "L" => Left,
            _ => Down,
        }
    }
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Move> {
    input.lines().map(Move::from_s).collect()
}

pub fn move_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    if head.0 == tail.0 && (head.1 - tail.1) == 2 {
        (tail.0, tail.1 + 1)
    } else if head.0 == tail.0 && (head.1 - tail.1) == -2 {
        (tail.0, tail.1 - 1)
    } else if head.1 == tail.1 && (head.0 - tail.0) == 2 {
        (tail.0 + 1, tail.1)
    } else if head.1 == tail.1 && (head.0 - tail.0) == -2 {
        (tail.0 - 1, tail.1)
    } else if ((head.0 - tail.0) >= 2 && (head.1 - tail.1) == 1)
        || ((head.0 - tail.0) == 1 && (head.1 - tail.1) >= 2)
    {
        (tail.0 + 1, tail.1 + 1)
    } else if ((head.0 - tail.0) <= -2 && (head.1 - tail.1) == -1)
        || ((head.0 - tail.0) == -1 && (head.1 - tail.1) <= -2)
    {
        (tail.0 - 1, tail.1 - 1)
    } else if ((head.0 - tail.0) <= -2 && (head.1 - tail.1) == 1)
        || ((head.0 - tail.0) == -1 && (head.1 - tail.1) >= 2)
    {
        (tail.0 - 1, tail.1 + 1)
    } else if ((head.0 - tail.0) >= 2 && (head.1 - tail.1) == -1)
        || ((head.0 - tail.0) == 1 && (head.1 - tail.1) <= -2)
    {
        (tail.0 + 1, tail.1 - 1)
    } else {
        tail
    }
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[Move]) -> usize {
    let mut seen_pos: HashSet<(i32, i32)> = HashSet::new();
    let mut head_pos: (i32, i32) = (0, 0);
    let mut tail_pos: (i32, i32) = (0, 0);

    for m in input {
        for _mag in 0..m.magnitude {
            use Dir::*;
            head_pos = match m.direction {
                Left => (head_pos.0, head_pos.1 - 1),
                Right => (head_pos.0, head_pos.1 + 1),
                Down => (head_pos.0 + 1, head_pos.1),
                Up => (head_pos.0 - 1, head_pos.1),
            };

            tail_pos = move_tail(head_pos, tail_pos);
            // println!("{} / {} - {:?}", _mag + 1, m.magnitude, &m);
            // dbg!(&head_pos);
            // dbg!(&tail_pos);
            seen_pos.insert(tail_pos);
        }
    }

    seen_pos.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let inp = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        let parsed_input = input_generator(inp);
        let solved = solve_part1(&parsed_input);
        assert_eq!(solved, 13);
    }
}
