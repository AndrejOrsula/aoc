use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day10, part1)]
#[must_use]
pub fn part1(input: &str) -> usize {
    let (map, max_elevation) = Map::parse(input);

    let mut goals: rustc_hash::FxHashMap<_, _> = max_elevation
        .iter()
        .map(|pos| {
            (
                *pos,
                [pos.0 * map.ncols() + pos.1]
                    .into_iter()
                    .collect::<smallvec::SmallVec<[_; 8]>>(),
            )
        })
        .collect();

    (1..10).rev().for_each(|i| {
        let mut next_goals = rustc_hash::FxHashMap::default();
        for (&(x, y), goal) in &goals {
            if x > 0 && map[(x - 1, y)] == i - 1 {
                next_goals
                    .entry((x - 1, y))
                    .or_insert(smallvec::SmallVec::new())
                    .extend(goal.clone());
            }
            if x < map.ncols() - 1 && map[(x + 1, y)] == i - 1 {
                next_goals
                    .entry((x + 1, y))
                    .or_insert(smallvec::SmallVec::new())
                    .extend(goal.clone());
            }
            if y > 0 && map[(x, y - 1)] == i - 1 {
                next_goals
                    .entry((x, y - 1))
                    .or_insert(smallvec::SmallVec::new())
                    .extend(goal.clone());
            }
            if y < map.nrows() - 1 && map[(x, y + 1)] == i - 1 {
                next_goals
                    .entry((x, y + 1))
                    .or_insert(smallvec::SmallVec::new())
                    .extend(goal.clone());
            }
        }
        goals = next_goals;
    });

    goals
        .values()
        .map(|goal| goal.iter().unique().count())
        .sum()
}

#[aoc(day10, part2)]
#[must_use]
pub fn part2(input: &str) -> usize {
    let (map, max_elevation) = Map::parse(input);

    let mut goals: rustc_hash::FxHashMap<_, _> =
        max_elevation.iter().map(|pos| (*pos, 1)).collect();

    (1..10).rev().for_each(|i| {
        let mut next_goals = rustc_hash::FxHashMap::default();
        for (&(x, y), goal) in &goals {
            if x > 0 && map[(x - 1, y)] == i - 1 {
                *next_goals.entry((x - 1, y)).or_insert(0) += goal;
            }

            if x < map.ncols() - 1 && map[(x + 1, y)] == i - 1 {
                *next_goals.entry((x + 1, y)).or_insert(0) += goal;
            }

            if y > 0 && map[(x, y - 1)] == i - 1 {
                *next_goals.entry((x, y - 1)).or_insert(0) += goal;
            }

            if y < map.nrows() - 1 && map[(x, y + 1)] == i - 1 {
                *next_goals.entry((x, y + 1)).or_insert(0) += goal;
            }
        }
        goals = next_goals;
    });

    goals.values().sum()
}

#[repr(transparent)]
#[derive(Clone, derive_more::Deref, derive_more::DerefMut)]
struct Map(nalgebra::DMatrix<u8>);

impl Map {
    fn parse(input: &str) -> (Self, smallvec::SmallVec<[(usize, usize); 256]>) {
        let input = input.as_bytes();
        let width = input.iter().position(|&c| c == b'\n').unwrap();
        let mut map = nalgebra::DMatrix::from_element(width, (input.len() - 1) / width, b'0');
        let mut max_elevation = smallvec::SmallVec::default();
        input.chunks(width + 1).enumerate().for_each(|(y, line)| {
            line.iter().enumerate().for_each(|(x, &b)| {
                if b.is_ascii_digit() {
                    {
                        map[(x, y)] = b - b'0';
                        if b == b'9' {
                            max_elevation.push((x, y));
                        }
                    }
                }
            });
        });
        (Self(map), max_elevation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
    "};

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(SAMPLE), 36);
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(part2(SAMPLE), 81);
    }
}
