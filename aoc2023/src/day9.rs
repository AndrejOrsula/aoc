use aoc_runner_derive::aoc;

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
#[must_use]
pub fn part1(input: &str) -> i32 {
    let input = parse(input);
    input
        .iter()
        .map(|seq| utils::predict_next_recursively(seq))
        .sum()
}

#[aoc(day9, part2)]
#[must_use]
pub fn part2(input: &str) -> i32 {
    let input = parse(input);
    input
        .clone()
        .iter_mut()
        .map(|arr| {
            (*arr).reverse();
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
    pub fn part1_example() {
        assert_eq!(part1(SAMPLE), 114);
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(part2(SAMPLE), 2);
    }
}
