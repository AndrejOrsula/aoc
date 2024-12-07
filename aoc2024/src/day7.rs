use aoc_runner_derive::aoc;
use rayon::prelude::*;

#[aoc(day7, part1)]
#[must_use]
pub fn part1(input: &str) -> u64 {
    input
        .par_lines()
        .filter_map(|line| {
            let (eq_result, eq_operands) = parse_line(line);
            is_solvable_part1(eq_operands, eq_result, 0).then_some(eq_result)
        })
        .sum()
}

#[aoc(day7, part2)]
#[must_use]
pub fn part2(input: &str) -> u64 {
    input
        .par_lines()
        .filter_map(|line| {
            let (eq_result, eq_operands) = parse_line(line);
            is_solvable_part2(eq_operands, eq_result, 0).then_some(eq_result)
        })
        .sum()
}

fn parse_line(line: &str) -> (u64, impl Iterator<Item = u64> + Clone + '_) {
    let (eq_result, eq_operands) = line.split_once(':').unwrap();
    (
        eq_result.parse().unwrap(),
        eq_operands
            .split_ascii_whitespace()
            .map(|v| v.parse().unwrap()),
    )
}

fn is_solvable_part1(
    mut eq_operands: impl std::iter::Iterator<Item = u64> + Clone,
    eq_result: u64,
    intermediate_result: u64,
) -> bool {
    if let Some(next_operand) = eq_operands.next() {
        is_solvable_part1(
            eq_operands.clone(),
            eq_result,
            intermediate_result + next_operand,
        ) || is_solvable_part1(eq_operands, eq_result, intermediate_result * next_operand)
    } else {
        eq_result == intermediate_result
    }
}

fn is_solvable_part2(
    mut eq_operands: impl std::iter::Iterator<Item = u64> + Clone,
    eq_result: u64,
    intermediate_result: u64,
) -> bool {
    if let Some(next_operand) = eq_operands.next() {
        is_solvable_part2(
            eq_operands.clone(),
            eq_result,
            intermediate_result + next_operand,
        ) || is_solvable_part2(
            eq_operands.clone(),
            eq_result,
            intermediate_result * next_operand,
        ) || is_solvable_part2(
            eq_operands,
            eq_result,
            intermediate_result * u64::pow(10, next_operand.to_string().len() as u32)
                + next_operand,
        )
    } else {
        eq_result == intermediate_result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
    "};

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(SAMPLE), 3749);
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(part2(SAMPLE), 11387);
    }
}
