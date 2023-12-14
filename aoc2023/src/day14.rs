use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day14)]
fn parse(input: &str) -> utils::Platform {
    let mut matrix = pathfinding::matrix::Matrix::new(
        input.lines().count(),
        input.lines().next().unwrap().len(),
        utils::Tile::Empty,
    );
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            matrix[(x, y)] = c.try_into().unwrap();
        });
    });
    utils::Platform::new(matrix)
}

#[aoc(day14, part1)]
fn part1(input: &utils::Platform) -> usize {
    let mut input = input.clone();
    input.slide_rocks_north();
    input.compute_load()
}

#[aoc(day14, part2)]
fn part2(input: &utils::Platform) -> usize {
    const TARGET_N_SPINS: usize = 1_000_000_000;

    let mut input = input.clone();
    let mut visited: rustc_hash::FxHashMap<utils::Platform, usize> =
        [(input.clone(), 0)].into_iter().collect();

    // Spin until the target is reached or a loop is detected
    for i in 1..TARGET_N_SPINS {
        use std::collections::hash_map::Entry;
        input.spin();
        // Determine the frequency of the spins
        match visited.entry(input.clone()) {
            Entry::Vacant(entry) => {
                entry.insert(i);
            }
            Entry::Occupied(entry) => {
                // Characterize the loop
                let loop_start = entry.get();
                let loop_frequency = i - loop_start;
                // Spin until the target is reached
                let spins_until_target = (TARGET_N_SPINS - loop_start) % loop_frequency;
                for _ in 0..spins_until_target {
                    input.spin();
                }
                break;
            }
        }
    }

    input.compute_load()
}

mod utils {
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub enum Tile {
        Empty,
        DynamicRock,
        StaticRock,
    }

    impl TryFrom<char> for Tile {
        type Error = &'static str;
        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                '.' => Ok(Self::Empty),
                'O' => Ok(Self::DynamicRock),
                '#' => Ok(Self::StaticRock),
                _ => Err("Unknown tile type"),
            }
        }
    }

    #[derive(Clone, PartialEq, Eq, Hash, derive_more::Deref, derive_more::DerefMut)]
    pub struct Platform(pathfinding::matrix::Matrix<Tile>);

    impl Platform {
        pub fn new(value: pathfinding::matrix::Matrix<Tile>) -> Self {
            Self(value)
        }

        pub fn compute_load(&self) -> usize {
            self.items()
                .map(|((_, y), tile)| {
                    tile.eq(&Tile::DynamicRock)
                        .then_some(self.rows - y)
                        .unwrap_or_default()
                })
                .sum()
        }

        pub fn slide_rocks_north(&mut self) {
            let mut n_rocks = 0;
            for x in 0..self.columns {
                for y in (0..self.rows).rev() {
                    match self[(x, y)] {
                        Tile::DynamicRock => {
                            self[(x, y)] = Tile::Empty;
                            n_rocks += 1;
                        }
                        Tile::StaticRock => {
                            if n_rocks > 0 {
                                let start_pos = y + 1;
                                for y_new in start_pos..(start_pos + n_rocks) {
                                    self[(x, y_new)] = Tile::DynamicRock;
                                }
                                n_rocks = 0;
                            }
                        }
                        Tile::Empty => {}
                    }
                }
                if n_rocks > 0 {
                    for y_new in 0..n_rocks {
                        self[(x, y_new)] = Tile::DynamicRock;
                    }
                    n_rocks = 0;
                }
            }
        }

        pub fn spin(&mut self) {
            // North -> West -> South -> East
            for _ in 0..4 {
                self.slide_rocks_north();
                self.rotate_ccw(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SAMPLE)), 136);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SAMPLE)), 64);
    }
}
