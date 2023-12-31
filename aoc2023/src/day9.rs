use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &[Vec<i32>]) -> i32 {
    input
        .iter()
        .map(|seq| utils::predict_next_recursively(seq))
        .sum()
}

#[aoc(day9, part2)]
fn part2(input: &[Vec<i32>]) -> i32 {
    input
        .to_owned()
        .iter_mut()
        .map(|arr| {
            arr.reverse();
            arr
        })
        .map(|seq| utils::predict_next_recursively(seq))
        .sum()
}

mod utils {
    pub fn predict_next_recursively(row: &[i32]) -> i32 {
        let next_row: Vec<_> = row.windows(2).map(|w| w[1] - w[0]).collect();

        row.last().unwrap()
            + next_row
                .iter()
                .any(|&number| number != 0)
                .then(|| predict_next_recursively(&next_row))
                .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SAMPLE)), 114);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SAMPLE)), 2);
    }
}
