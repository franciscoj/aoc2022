use aoc_runner_derive::aoc;

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> usize {
    let msg: Vec<char> = input.chars().collect();
    let mut is_marker = false;

    input.chars().fold(1 as usize, |acc, _x| {
        if acc < 4 {
            acc + 1
        } else {
            if is_marker {
                acc
            } else {
                let chars = &msg[acc - 4..acc];
                is_marker = chars[0] != chars[1]
                    && chars[1] != chars[2]
                    && chars[2] != chars[3]
                    && chars[0] != chars[2]
                    && chars[0] != chars[3]
                    && chars[1] != chars[3];

                if is_marker {
                    acc
                } else {
                    acc + 1
                }
            }
        }
    })
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> usize {
    let msg: Vec<char> = input.chars().collect();
    let mut is_marker = false;

    input.chars().fold(1 as usize, |acc, _x| {
        if acc < 14 {
            acc + 1
        } else {
            if is_marker {
                acc
            } else {
                let mut chars = msg[acc - 14..acc].to_vec().clone();
                chars.sort_unstable();
                chars.dedup();
                is_marker = chars.len() == 14;

                if is_marker {
                    acc
                } else {
                    acc + 1
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day6_part1() {
        assert_eq!(7, solve_part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(5, solve_part1("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, solve_part1("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, solve_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(11, solve_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn test_day6_part2() {
        assert_eq!(19, solve_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(23, solve_part2("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(23, solve_part2("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(29, solve_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(26, solve_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }
}
