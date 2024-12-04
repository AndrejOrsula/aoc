use aoc_runner_derive::aoc;
use strum::IntoEnumIterator;

fn parse(input: &str) -> pathfinding::matrix::Matrix<char> {
    let mut matrix = pathfinding::matrix::Matrix::new(
        input.lines().count(),
        input.lines().next().unwrap().len(),
        ' ',
    );
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            matrix[(x, y)] = c;
        });
    });
    matrix
}

#[aoc(day4, part1)]
#[must_use]
pub fn part1(input: &str) -> usize {
    let matrix = parse(input);
    matrix
        .items()
        .filter_map(|((x, y), &tile)| (tile == 'X').then_some((x, y)))
        .map(|(x, y)| {
            Direction::iter()
                .filter(|direction| {
                    ['M', 'A', 'S']
                        .into_iter()
                        .try_fold((x, y), |(x, y), letter| {
                            direction
                                .propagate(&matrix, (x, y))
                                .and_then(|(x, y)| (matrix[(x, y)] == letter).then_some((x, y)))
                        })
                        .is_some()
                })
                .count()
        })
        .sum()
}

#[aoc(day4, part2)]
#[must_use]
pub fn part2(input: &str) -> usize {
    let matrix = parse(input);
    matrix
        .items()
        .filter_map(|((x, y), &tile)| (tile == 'A').then_some((x, y)))
        .filter(|&(x, y)| {
            [
                (Direction::NorthWest, Direction::SouthEast),
                (Direction::NorthEast, Direction::SouthWest),
            ]
            .into_iter()
            .all(|(dir1, dir2)| {
                dir1.propagate(&matrix, (x, y)).and_then(|(x1, y1)| {
                    dir2.propagate(&matrix, (x, y)).map(|(x2, y2)| {
                        (matrix[(x1, y1)] == 'M' && matrix[(x2, y2)] == 'S')
                            || (matrix[(x1, y1)] == 'S' && matrix[(x2, y2)] == 'M')
                    })
                }) == Some(true)
            })
        })
        .count()
}

#[derive(strum::EnumIter)]
pub enum Direction {
    North,
    NorthWest,
    West,
    SouthWest,
    South,
    SouthEast,
    East,
    NorthEast,
}

impl Direction {
    fn propagate(
        &self,
        matrix: &pathfinding::matrix::Matrix<char>,
        (x, y): (usize, usize),
    ) -> Option<(usize, usize)> {
        Some(match self {
            Self::North => (x, y.checked_sub(1)?),
            Self::NorthWest => (x.checked_sub(1)?, y.checked_sub(1)?),
            Self::West => (x.checked_sub(1)?, y),
            Self::SouthWest => (x.checked_sub(1)?, {
                let y = y + 1;
                (y < matrix.rows).then_some(y)?
            }),
            Self::South => (x, {
                let y = y + 1;
                (y < matrix.rows).then_some(y)?
            }),
            Self::SouthEast => (
                {
                    let x = x + 1;
                    (x < matrix.columns).then_some(x)?
                },
                {
                    let y = y + 1;
                    (y < matrix.rows).then_some(y)?
                },
            ),
            Self::East => (
                {
                    let x = x + 1;
                    (x < matrix.columns).then_some(x)?
                },
                y,
            ),
            Self::NorthEast => (
                {
                    let x = x + 1;
                    (x < matrix.columns).then_some(x)?
                },
                y.checked_sub(1)?,
            ),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    "};

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(SAMPLE), 18);
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(part2(SAMPLE), 9);
    }
}
