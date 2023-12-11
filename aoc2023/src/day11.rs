use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day11, part1)]
fn parse1(input: &str) -> Vec<(usize, usize)> {
    utils::parse_expanded_galaxies(input, 1)
}

#[aoc_generator(day11, part2)]
fn parse2(input: &str) -> Vec<(usize, usize)> {
    utils::parse_expanded_galaxies(input, 999_999)
}

#[aoc(day11, part1)]
fn part1(input: &[(usize, usize)]) -> usize {
    utils::sum_galactic_distances(input)
}

#[aoc(day11, part2)]
fn part2(input: &[(usize, usize)]) -> usize {
    utils::sum_galactic_distances(input)
}

mod utils {
    pub fn parse_expanded_galaxies(input: &str, expansion_rate: usize) -> Vec<(usize, usize)> {
        let expanding_rows = find_expanding_rows(input);
        let expanding_cols = find_expanding_cols(input);
        let galaxies = find_galaxies(input);

        expand_universe(&galaxies, &expanding_rows, &expanding_cols, expansion_rate)
    }

    pub fn sum_galactic_distances(galaxies: &[(usize, usize)]) -> usize {
        use itertools::Itertools;
        galaxies
            .iter()
            .combinations(2)
            .map(|pair| {
                // Manhattan distance
                let (galaxy_a, galaxy_b) = (pair[0], pair[1]);
                let (dx, dy) = (
                    galaxy_a.0.abs_diff(galaxy_b.0),
                    galaxy_a.1.abs_diff(galaxy_b.1),
                );
                dx + dy
            })
            .sum()
    }

    fn find_expanding_rows(input: &str) -> Vec<usize> {
        input
            .lines()
            .enumerate()
            .filter(|(_, line)| line.chars().all(|c| c == '.'))
            .map(|(x, _)| x)
            .collect()
    }

    fn find_expanding_cols(input: &str) -> Vec<usize> {
        let mut expanding_cols = Vec::new();
        'outer: for y in 0..input.lines().next().unwrap().chars().count() {
            for x in 0..input.lines().count() {
                if input.lines().nth(x).unwrap().chars().nth(y).unwrap() == '#' {
                    continue 'outer;
                }
            }
            expanding_cols.push(y);
        }
        expanding_cols
    }

    fn find_galaxies(input: &str) -> Vec<(usize, usize)> {
        let mut galaxies = Vec::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    galaxies.push((x, y));
                }
            }
        }
        galaxies
    }

    fn expand_universe(
        galaxies: &[(usize, usize)],
        expanding_rows: &[usize],
        expanding_cols: &[usize],
        expasion_rate: usize,
    ) -> Vec<(usize, usize)> {
        let (x_limit, y_limit) = (
            galaxies.iter().max_by_key(|(x, _)| x).unwrap().0 + 1,
            galaxies.iter().max_by_key(|(_, y)| y).unwrap().1 + 1,
        );

        let mut expanded_galaxies = Vec::new();
        let mut n_expanded_rows = 0;
        let mut n_expanded_cols = 0;
        for y in 0..y_limit {
            if expanding_rows.contains(&y) {
                n_expanded_rows += expasion_rate;
                continue;
            }
            for x in 0..x_limit {
                if expanding_cols.contains(&x) {
                    n_expanded_cols += expasion_rate;
                    continue;
                }
                if galaxies.contains(&(x, y)) {
                    expanded_galaxies.push((x + n_expanded_cols, y + n_expanded_rows));
                }
            }
            n_expanded_cols = 0;
        }
        expanded_galaxies
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse1(SAMPLE)), 374);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&utils::parse_expanded_galaxies(SAMPLE, 9)), 1030);
        assert_eq!(part2(&utils::parse_expanded_galaxies(SAMPLE, 99)), 8410);
    }
}
