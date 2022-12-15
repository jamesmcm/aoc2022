use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

pub enum LogLine {
    Comm(Command),
    Response(FileDir),
}

pub enum Command {
    Ls,
    Cd(String),
}

pub enum FileDir {
    File(i32, String),
    Dir(String),
}

pub fn parse_command(l: &str) -> Command {
    let v: Vec<String> = l.split(' ').map(|x| x.to_owned()).collect();
    if v[1] == "ls" {
        Command::Ls
    } else {
        Command::Cd(v[2].clone())
    }
}

pub fn parse_file_dir(l: &str) -> FileDir {
    let v: Vec<String> = l.split(' ').map(|x| x.to_owned()).collect();
    if v[0] == "dir" {
        FileDir::Dir(v[1].clone())
    } else {
        FileDir::File(v[0].parse().unwrap(), v[1].clone())
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<LogLine> {
    input
        .lines()
        .map(|l| {
            if l.starts_with('$') {
                LogLine::Comm(parse_command(l))
            } else {
                LogLine::Response(parse_file_dir(l))
            }
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[LogLine]) -> i32 {
    let mut parents: Vec<String> = vec!["root".to_owned()];
    let mut dirsizes: HashMap<String, i32> = HashMap::new();
    dirsizes.insert("root".to_owned(), 0);

    for l in input {
        match l {
            LogLine::Comm(Command::Ls) => {}
            LogLine::Comm(Command::Cd(subdir)) => {
                if subdir == "/" {
                    // Ignore since we add at start - only appears at start
                } else if subdir == ".." {
                    parents.pop();
                } else {
                    parents.push(subdir.to_owned());
                    dirsizes.entry(subdir.to_owned()).or_insert(0);
                }
            }

            LogLine::Response(FileDir::Dir(_subdir)) => {}
            LogLine::Response(FileDir::File(size, _name)) => {
                for p in parents.iter() {
                    dirsizes
                        .entry(p.to_owned())
                        .and_modify(|v| *v += size)
                        .or_insert(*size);
                }
            }
        }
    }

    dirsizes
        .into_iter()
        .filter(|x| x.1 <= 100_000)
        .map(|x| x.1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let inp = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        let parsed_input = input_generator(inp);
        assert_eq!(solve_part1(&parsed_input), 95437);
    }
}
