use std::fmt;
use std::str::FromStr;

use aoc_runner_derive::aoc;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
struct Step(u32, u32, u32);

#[derive(Debug, PartialEq, Eq)]
struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid step format")
    }
}

impl FromStr for Step {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parser = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

        if let Some(caps) = parser.captures(s) {
            let size = caps[1].parse::<u32>().unwrap_or(0);
            let start = caps[2].parse::<u32>().unwrap_or(0);
            let end = caps[3].parse::<u32>().unwrap_or(0);

            Ok(Step(size, start, end))
        } else {
            println!("{}", s);

            Err(ParseError)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Stack {
    crates: Vec<char>,
}

impl Stack {
    fn push(&mut self, el: Option<char>) {
        if let Some(c) = el {
            self.crates.push(c);
        }
    }

    fn pop(&mut self) -> Option<char> {
        self.crates.pop()
    }

    fn split_off(&mut self, at: u32) -> Vec<char> {
        self.crates.split_off(at as usize)
    }

    fn append(&mut self, crates: &mut Vec<char>) {
        self.crates.append(crates)
    }

    fn top(&self) -> char {
        *self.crates.last().unwrap_or(&'\0')
    }

    fn len(&self) -> u32 {
        self.crates.len() as u32
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Cargo {
    stacks: Vec<Stack>,
}

impl Cargo {
    fn apply(&mut self, steps: Vec<Step>) {
        for step in steps {
            let Step(size, start, end) = step;

            for _n in 0..size {
                let start_stack = &mut self.stacks[(start - 1) as usize];
                let el = start_stack.pop();
                let end_stack = &mut self.stacks[(end - 1) as usize];
                end_stack.push(el);
            }
        }
    }

    fn apply_9001(&mut self, steps: Vec<Step>) {
        for step in steps {
            let Step(size, start, end) = step;

            let split_at = self.stacks[(start - 1) as usize].len() - size;
            let start_stack = &mut self.stacks[(start - 1) as usize];
            let mut els = start_stack.split_off(split_at);
            let end_stack = &mut self.stacks[(end - 1) as usize];
            end_stack.append(&mut els);
        }
    }

    fn tops(&self) -> String {
        let tops = self.stacks.as_slice().iter().map(|s| s.top());

        String::from_iter(tops)
    }
}

impl FromStr for Cargo {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split("\n").collect();
        let columns = lines[0].len() / 4 + 1;
        let mut cargo: Vec<Vec<char>> = vec![];
        // prepare the empty stacks
        (0..columns).for_each(|_n| cargo.push(vec![]));

        // Parse each column line by line, if it has a letter, add it to the stack. If not just
        // ignore it.
        lines.iter().rev().for_each(|line| {
            for n in 0..columns {
                let idx: usize = 1 + n * 4;

                if let Some(el) = line.chars().nth(idx) {
                    if let 'A'..='Z' = el {
                        cargo[n].push(el)
                    }
                }
            }
        });

        let stacks = cargo
            .iter()
            .map(|crates| Stack {
                crates: crates.to_vec(),
            })
            .collect::<Vec<Stack>>();

        Ok(Cargo { stacks })
    }
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> String {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let mut cargo = Cargo::from_str(parts[0]).unwrap();
    let steps: Vec<Step> = parts[1]
        .split("\n")
        .map(|line| Step::from_str(line).unwrap())
        .collect();

    cargo.apply(steps);
    cargo.tops()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> String {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let mut cargo = Cargo::from_str(parts[0]).unwrap();
    let steps: Vec<Step> = parts[1]
        .split("\n")
        .map(|line| Step::from_str(line).unwrap())
        .collect();

    cargo.apply_9001(steps);
    cargo.tops()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day5_part1() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        assert_eq!(solve_part1(input), "CMZ");
    }

    #[test]
    fn test_day5_part2() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        assert_eq!(solve_part2(input), "MCD");
    }

    #[test]
    fn test_step_from_str() {
        assert_eq!(Step::from_str("move 10 from 8 to 1"), Ok(Step(10, 8, 1)));
        assert_eq!(Step::from_str("move 0 from 0 to 0"), Ok(Step(0, 0, 0)));
        assert_eq!(Step::from_str("move 1 from 1 to 1"), Ok(Step(1, 1, 1)));
        assert_eq!(Step::from_str(""), Err(ParseError));
    }

    #[test]
    fn test_cargo_from_str() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 ";

        assert_eq!(
            Cargo::from_str(input),
            Ok(Cargo {
                stacks: vec![
                    Stack {
                        crates: vec!['Z', 'N']
                    },
                    Stack {
                        crates: vec!['M', 'C', 'D']
                    },
                    Stack { crates: vec!['P'] }
                ]
            })
        )
    }
}
