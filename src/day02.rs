use aoc_runner_derive::aoc;

#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let sum = input
        .split("\n")
        .map(|line| -> u32 {
            match line {
                "A X" => 1 + 3,
                "A Y" => 2 + 6,
                "A Z" => 3 + 0,
                "B X" => 1 + 0,
                "B Y" => 2 + 3,
                "B Z" => 3 + 6,
                "C X" => 1 + 6,
                "C Y" => 2 + 0,
                "C Z" => 3 + 3,
                _ => 0,
            }
        })
        .sum();

    sum
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let sum = input
        .split("\n")
        .map(|line| -> u32 {
            match line {
                "A X" => 3 + 0,
                "A Y" => 1 + 3,
                "A Z" => 2 + 6,
                "B X" => 1 + 0,
                "B Y" => 2 + 3,
                "B Z" => 3 + 6,
                "C X" => 2 + 0,
                "C Y" => 3 + 3,
                "C Z" => 1 + 6,
                _ => 0,
            }
        })
        .sum();

    sum
}
