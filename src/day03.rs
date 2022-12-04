use aoc_runner_derive::aoc;

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let priorities: u32 = input
        .split("\n")
        .map(|sack| {
            let left = &sack[..sack.len() / 2];
            let right = &sack[sack.len() / 2..];

            let misplaced_item = left
                .chars()
                .find(|c| right.chars().any(|right_c| right_c == *c));

            if let Some(priority) = misplaced_item {
                to_priority(priority)
            } else {
                0
            }
        })
        .sum();

    priorities
}

fn to_priority(i: char) -> u32 {
    let raw = i as u32;

    if raw <= 90 {
        raw - 38
    } else {
        raw - 96
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day3_part1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(157, solve_part1(input));
    }

    #[test]
    fn test_to_priority() {
        assert_eq!(27, to_priority('A'));
        assert_eq!(52, to_priority('Z'));
        assert_eq!(1, to_priority('a'));
        assert_eq!(26, to_priority('z'));
    }
}
