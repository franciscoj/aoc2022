use aoc_runner_derive::aoc;

#[derive(Debug, Copy, Clone)]
struct Section(u32, u32);

impl Section {
    fn contains(&self, s: &Section) -> bool {
        self.0 <= s.0 && self.1 >= s.1
    }

    fn overlaps(&self, s: &Section) -> bool {
        self.0 <= s.0 && s.0 <= self.1 || s.1 <= self.1 && self.0 <= s.1
    }
}

struct Pair(Section, Section);

impl Pair {
    fn from_line(line: &str) -> Pair {
        let sections = line
            .split(",")
            .map(|section| {
                let parts = section
                    .split("-")
                    .map(|part| -> u32 { part.parse::<u32>().unwrap_or(0) })
                    .collect::<Vec<u32>>();

                Section(parts[0], parts[1])
            })
            .collect::<Vec<Section>>();

        let left = sections[0];
        let right = sections[1];

        Pair(left, right)
    }

    fn contains(&self) -> bool {
        let Pair(left, right) = self;

        left.contains(&right) || right.contains(&left)
    }

    fn overlaps(&self) -> bool {
        self.0.overlaps(&self.1) || self.1.overlaps(&self.0)
    }
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &str) -> usize {
    input
        .split("\n")
        .map(|line| Pair::from_line(line))
        .filter(|pair| pair.contains())
        .count()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &str) -> usize {
    input
        .split("\n")
        .map(|line| Pair::from_line(line))
        .filter(|pair| pair.overlaps())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day4_part1() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(2, solve_part1(input));
    }

    #[test]
    fn test_day4_part2() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(4, solve_part2(input));
    }

    #[test]
    fn test_section_contains() {
        let a = Section(1, 1);
        let b = Section(1, 1);
        let c = Section(1, 2);
        let d = Section(3, 8);

        assert!(a.contains(&b));
        assert!(c.contains(&b));
        assert!(!a.contains(&c));
        assert!(!d.contains(&c));
    }

    #[test]
    fn test_section_overlaps() {
        let a = Section(1, 2);
        let b = Section(2, 3);
        let c = Section(1, 2);
        let d = Section(3, 8);

        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));
        assert!(!c.overlaps(&d));
        assert!(!d.overlaps(&c));
    }

    #[test]
    fn test_pair_contains() {
        let a = Pair(Section(1, 1), Section(1, 1));
        let b = Pair(Section(1, 2), Section(1, 1));
        let c = Pair(Section(1, 2), Section(2, 3));

        assert!(a.contains());
        assert!(b.contains());
        assert!(!c.contains());
    }

    #[test]
    fn test_pair_overlaps() {
        let a = Pair(Section(1, 1), Section(1, 1));
        let b = Pair(Section(1, 2), Section(1, 1));
        let c = Pair(Section(1, 2), Section(3, 4));

        assert!(a.overlaps());
        assert!(b.overlaps());
        assert!(!c.overlaps());
    }

    #[test]
    fn test_pair_from_line() {
        let Pair(a, b) = Pair::from_line("1-2,3-4");

        assert_eq!(a.0, 1);
        assert_eq!(a.1, 2);
        assert_eq!(b.0, 3);
        assert_eq!(b.1, 4);
    }
}
