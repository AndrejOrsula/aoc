use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day18, part1)]
fn parse1(input: &str) -> Vec<(utils::Direction, u32)> {
    use itertools::Itertools;

    input
        .lines()
        .map(|line| {
            let (direction, distance, _color) =
                line.split_ascii_whitespace().collect_tuple().unwrap();
            let direction = utils::Direction::try_from(direction.chars().next().unwrap()).unwrap();
            let distance = distance.parse().unwrap();
            (direction, distance)
        })
        .collect()
}

#[aoc_generator(day18, part2)]
fn parse2(input: &str) -> Vec<(utils::Direction, u32)> {
    use itertools::Itertools;

    input
        .lines()
        .map(|line| {
            let (_, _, instruction) = line.split_ascii_whitespace().collect_tuple().unwrap();
            let instruction = instruction
                .strip_prefix("(#")
                .unwrap()
                .strip_suffix(')')
                .unwrap();
            let direction =
                utils::Direction::try_from(instruction.chars().last().unwrap()).unwrap();
            let distance = u32::from_str_radix(&instruction[..instruction.len() - 1], 16).unwrap();
            (direction, distance)
        })
        .collect()
}

#[aoc(day18, part1)]
fn part1(input: &[(utils::Direction, u32)]) -> u64 {
    utils::compute_area(input)
}

#[aoc(day18, part2)]
fn part2(input: &[(utils::Direction, u32)]) -> u64 {
    utils::compute_area(input)
}

mod utils {
    pub enum Direction {
        Right,
        Down,
        Left,
        Up,
    }

    impl TryFrom<char> for Direction {
        type Error = &'static str;
        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                'R' | '0' => Ok(Direction::Right),
                'D' | '1' => Ok(Direction::Down),
                'L' | '2' => Ok(Direction::Left),
                'U' | '3' => Ok(Direction::Up),
                _ => Err("Invalid direction"),
            }
        }
    }

    pub fn compute_area(instruction: &[(Direction, u32)]) -> u64 {
        let mut area = 0;
        let mut vertical_offset = 0;
        let mut perimeter = 0;
        for (direction, distance) in instruction {
            match direction {
                Direction::Right => {
                    area += vertical_offset * i64::from(*distance);
                }
                Direction::Down => {
                    vertical_offset += i64::from(*distance);
                }
                Direction::Left => {
                    area -= vertical_offset * i64::from(*distance);
                }
                Direction::Up => {
                    vertical_offset -= i64::from(*distance);
                }
            }
            perimeter += u64::from(*distance);
        }
        area.unsigned_abs() + (perimeter / 2) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse1(SAMPLE)), 62);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse2(SAMPLE)), 952_408_144_115);
    }
}
