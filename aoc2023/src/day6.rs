use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn parse(input: &str) -> smallvec::SmallVec<[utils::RaceData; 4]> {
    use itertools::Itertools;

    let (times, distances) = input
        .lines()
        .map(|line| line.split(':').last().unwrap().split_ascii_whitespace())
        .collect_tuple()
        .unwrap();
    times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| utils::RaceData {
            time: time.to_owned(),
            distance: distance.to_owned(),
        })
        .collect()
}

#[aoc(day6, part1)]
fn part1(input: &[utils::RaceData]) -> u64 {
    input.iter().map(utils::RaceData::n_record_breaks).product()
}

#[aoc(day6, part2)]
fn part2(input: &[utils::RaceData]) -> u64 {
    utils::RaceData::merge(input).n_record_breaks1()
}

mod utils {
    pub struct RaceData {
        pub time: String,
        pub distance: String,
    }

    impl RaceData {
        /// Solver using brute force (faster than the quadratic formula for small inputs).
        pub fn n_record_breaks(&self) -> u64 {
            let (time, distance) = (self.time.parse().unwrap(), self.distance.parse().unwrap());
            match (1..time).find(|j| j * (time - j) > distance) {
                Some(first_record) => 1 + time - (2 * first_record),
                _ => 1,
            }
        }

        /// Solver based on quadratic formula (much faster than brute force for large inputs).
        pub fn n_record_breaks1(&self) -> u64 {
            let (time, distance) = (
                self.time.parse::<u64>().unwrap(),
                self.distance.parse::<u64>().unwrap(),
            );
            2 * num_integer::sqrt(time.pow(2) / 4 - distance - 1) + (time % 2) + 1
        }

        pub fn merge(sequence: &[Self]) -> Self {
            let (time, distance) = sequence.iter().fold(
                Default::default(),
                |(time, distance): (String, String), data| {
                    (time + &data.time, distance + &data.distance)
                },
            );
            Self { time, distance }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SAMPLE)), 288);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SAMPLE)), 71503);
    }
}
