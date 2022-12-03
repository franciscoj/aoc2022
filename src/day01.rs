use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let max: Option<u32> = input
        .split("\n\n")
        .map(|e| {
            e.split("\n")
                .map(|n| -> u32 { n.parse::<u32>().unwrap_or(0) })
                .sum()
        })
        .max();

    max.unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let mut elfs = input
        .split("\n\n")
        .map(|e| {
            e.split("\n")
                .map(|n| -> u32 { n.parse::<u32>().unwrap_or(0) })
                .sum()
        })
        .collect::<Vec<u32>>();
    elfs.sort();
    elfs[elfs.len() - 3..].to_vec().iter().sum()
}
