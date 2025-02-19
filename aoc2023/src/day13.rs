use aoc_runner_derive::aoc;

fn parse(input: &str) -> Vec<utils::Pattern> {
    input
        .split("\n\n")
        .map(|s| utils::Pattern::new(&s))
        .collect()
}

#[aoc(day13, part1)]
#[must_use]
pub fn part1(input: &str) -> usize {
    let input = parse(input);
    input
        .iter()
        .map(|pattern| {
            pattern.solve(|start, end| start.iter().rev().zip(end.iter()).all(|(s1, s2)| s1 == s2))
        })
        .sum()
}

#[aoc(day13, part2)]
#[must_use]
pub fn part2(input: &str) -> usize {
    use rayon::prelude::*;

    let input = parse(input);
    input
        .iter()
        .par_bridge()
        .map(|pattern| {
            pattern.solve(|start, end| {
                1 == start
                    .iter()
                    .rev()
                    .zip(end.iter())
                    .flat_map(|(s1, s2)| s1.chars().zip(s2.chars()).filter(|(c1, c2)| c1 != c2))
                    .count()
            })
        })
        .sum()
}

mod utils {
    #[derive(derive_more::Deref)]
    pub struct Pattern(String);

    impl Pattern {
        pub fn new(value: &impl ToString) -> Self {
            Self(value.to_string())
        }

        pub fn solve<F>(&self, predicate: F) -> usize
        where
            F: Fn(&[String], &[String]) -> bool,
        {
            100 * find_reflection(&self.horizontal_lines(), &predicate).unwrap_or_default()
                + find_reflection(&self.vertical_lines(), &predicate).unwrap_or_default()
        }

        fn horizontal_lines(&self) -> Vec<String> {
            self.lines().map(std::borrow::ToOwned::to_owned).collect()
        }

        fn vertical_lines(&self) -> Vec<String> {
            (0..self.lines().next().unwrap().len())
                .map(|i| self.lines().flat_map(|line| line[i..=i].chars()).collect())
                .collect()
        }
    }

    fn find_reflection<T, F>(input: &[T], predicate: F) -> Option<usize>
    where
        T: Eq,
        F: Fn(&[T], &[T]) -> bool,
    {
        (1..input.len()).find(|&i| predicate(&input[..i], &input[i..]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
    "};

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(SAMPLE), 405);
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(part2(SAMPLE), 400);
    }
}
