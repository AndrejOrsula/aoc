use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day12)]
fn parse(input: &str) -> Vec<utils::SpringSequence> {
    input
        .lines()
        .map(|line| {
            let (springs, contiguously_damaged_springs) =
                line.split_once(char::is_whitespace).unwrap();
            utils::SpringSequence {
                springs: springs.chars().map(|c| c.try_into().unwrap()).collect(),
                contiguously_damaged_springs: contiguously_damaged_springs
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect(),
            }
        })
        .collect()
}

#[aoc(day12, part1)]
fn part1(input: &[utils::SpringSequence]) -> usize {
    input
        .iter()
        .map(utils::SpringSequence::discover_arrangements)
        .sum()
}

#[aoc(day12, part2)]
fn part2(input: &[utils::SpringSequence]) -> usize {
    input
        .iter()
        .map(|sequence| {
            // Repeat the input while joining with an unknown spring
            const N_REPEATS: usize = 5;
            utils::SpringSequence {
                springs: itertools::Itertools::intersperse(
                    [&sequence.springs; N_REPEATS].into_iter(),
                    &vec![utils::SpringType::Unknown],
                )
                .flatten()
                .copied()
                .collect(),
                contiguously_damaged_springs: sequence
                    .contiguously_damaged_springs
                    .repeat(N_REPEATS),
            }
        })
        .map(|sequence| sequence.discover_arrangements())
        .sum()
}

mod utils {
    pub struct SpringSequence {
        pub springs: Vec<SpringType>,
        pub contiguously_damaged_springs: Vec<usize>,
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum SpringType {
        Operational,
        Damaged,
        Unknown,
    }

    impl TryFrom<char> for SpringType {
        type Error = &'static str;
        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                '.' => Ok(Self::Operational),
                '#' => Ok(Self::Damaged),
                '?' => Ok(Self::Unknown),
                _ => Err("Unknown spring type"),
            }
        }
    }

    impl SpringSequence {
        pub fn discover_arrangements(&self) -> usize {
            let last = self.contiguously_damaged_springs.last().unwrap();
            self.contiguously_damaged_springs
                .iter()
                .rev()
                .skip(1)
                .fold(
                    (0..self.springs.len())
                        .map(|i| {
                            usize::from(
                                !(last + i > self.springs.len()
                                    || self.springs[i..i + last]
                                        .iter()
                                        .any(|&spring| spring == SpringType::Operational)
                                    || self.springs[i + last..]
                                        .iter()
                                        .any(|&spring| spring == SpringType::Damaged)
                                    || i != 0 && self.springs[i - 1] == SpringType::Damaged),
                            )
                        })
                        .collect::<Vec<_>>(),
                    |counts, run| {
                        (0..self.springs.len())
                            .map(|i| {
                                if run + i >= self.springs.len()
                                    || i != 0 && self.springs[i - 1] == SpringType::Damaged
                                    || self.springs[i..run + i]
                                        .iter()
                                        .any(|&spring| spring == SpringType::Operational)
                                    || self.springs[run + i] == SpringType::Damaged
                                {
                                    0
                                } else {
                                    counts[run + i + 1..]
                                        .iter()
                                        .zip(
                                            self.springs[run + i + 1..]
                                                .iter()
                                                .take_while(|&&spring| {
                                                    spring != SpringType::Damaged
                                                })
                                                .chain([&SpringType::Damaged]),
                                        )
                                        .map(|(&count, _)| count)
                                        .sum()
                                }
                            })
                            .collect()
                    },
                )
                .into_iter()
                .zip(
                    self.springs
                        .clone()
                        .into_iter()
                        .take_while(|&spring| spring != SpringType::Damaged)
                        .chain([SpringType::Damaged]),
                )
                .map(|(count, _)| count)
                .sum()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SAMPLE)), 21);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SAMPLE)), 525_152);
    }
}
