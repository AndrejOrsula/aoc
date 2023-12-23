use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day23)]
fn parse(input: &str) -> utils::Map {
    let mut matrix = pathfinding::matrix::Matrix::new(
        input.lines().count(),
        input.lines().next().unwrap().len(),
        utils::Tile::Path,
    );
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            matrix[(x, y)] = c.try_into().unwrap();
        });
    });
    let start_position = (1, 0);
    let end_position = (matrix.columns - 2, matrix.rows - 1);
    utils::Map {
        matrix,
        start_position,
        end_position,
    }
}

#[aoc(day23, part1)]
fn part1(input: &utils::Map) -> usize {
    input.longest_path_len(true)
}

#[aoc(day23, part2)]
fn part2(input: &utils::Map) -> usize {
    input.longest_path_len(false)
}

mod utils {
    use itertools::Itertools;
    use strum::{EnumIter, IntoEnumIterator};

    pub struct Map {
        pub matrix: pathfinding::matrix::Matrix<Tile>,
        pub start_position: (usize, usize),
        pub end_position: (usize, usize),
    }

    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Tile {
        Path = b'.',
        Forest = b'#',
        SlopeNorth = b'^',
        SlopeWest = b'<',
        SlopeSouth = b'v',
        SlopeEast = b'>',
    }

    impl TryFrom<char> for Tile {
        type Error = &'static str;
        fn try_from(c: char) -> Result<Self, Self::Error> {
            match c {
                '.' => Ok(Self::Path),
                '#' => Ok(Self::Forest),
                '^' => Ok(Self::SlopeNorth),
                '<' => Ok(Self::SlopeWest),
                'v' => Ok(Self::SlopeSouth),
                '>' => Ok(Self::SlopeEast),
                _ => Err("Invalid tile"),
            }
        }
    }

    #[derive(PartialEq, Eq, Hash, Copy, Clone, EnumIter)]
    enum Direction {
        North,
        West,
        South,
        East,
    }

    impl TryFrom<Tile> for Direction {
        type Error = &'static str;
        fn try_from(tile: Tile) -> Result<Self, Self::Error> {
            match tile {
                Tile::SlopeNorth => Ok(Self::North),
                Tile::SlopeWest => Ok(Self::West),
                Tile::SlopeSouth => Ok(Self::South),
                Tile::SlopeEast => Ok(Self::East),
                _ => Err("Invalid tile"),
            }
        }
    }

    type SuccessorLUT =
        rustc_hash::FxHashMap<(usize, usize), rustc_hash::FxHashMap<(usize, usize), usize>>;

    impl Map {
        pub fn longest_path_len(&self, include_slopes: bool) -> usize {
            // Construct LUT of successors
            let mut successor_lut: SuccessorLUT = (0..self.matrix.rows)
                .cartesian_product(0..self.matrix.columns)
                .filter(|&position| self.matrix[position] != Tile::Forest)
                .map(|position| {
                    let successors = self
                        .successors(position, include_slopes)
                        .into_iter()
                        .map(|successor| (successor, 1))
                        .collect();
                    (position, successors)
                })
                .collect();
            successor_lut
                .clone()
                .iter()
                .filter(|(_, successors)| successors.len() == 2)
                .map(|(&position, _)| position)
                .for_each(|position| {
                    let successors = successor_lut.remove(&position).unwrap();
                    let ((position1, distance1), (position2, distance2)) =
                        successors.iter().collect_tuple().unwrap();
                    let position1_entry = successor_lut.get_mut(position1).unwrap();
                    position1_entry.remove(&position);
                    position1_entry.insert(*position2, distance1 + distance2);
                    let position2_entry = successor_lut.get_mut(position2).unwrap();
                    position2_entry.remove(&position);
                    position2_entry.insert(*position1, distance1 + distance2);
                });

            // Find longest path with depth-first search
            dfs_longest_path(self.start_position, &successor_lut, |&position| {
                position == self.end_position
            })
            .unwrap()
        }

        fn successors(
            &self,
            position: (usize, usize),
            include_slopes: bool,
        ) -> Vec<(usize, usize)> {
            Direction::iter()
                .filter_map(
                    |direction| match Direction::try_from(self.matrix[position]) {
                        Ok(slope) if include_slopes && direction != slope => None,
                        _ => match direction {
                            Direction::North => {
                                position.1.checked_sub(1).map(|next_y| (position.0, next_y))
                            }
                            Direction::West => {
                                position.0.checked_sub(1).map(|next_x| (next_x, position.1))
                            }
                            Direction::South => {
                                let next_y = position.1 + 1;
                                (next_y < self.matrix.rows).then_some((position.0, next_y))
                            }
                            Direction::East => {
                                let next_x = position.0 + 1;
                                (next_x < self.matrix.columns).then_some((next_x, position.1))
                            }
                        }
                        .and_then(|next_position| {
                            (self.matrix[next_position] != Tile::Forest).then_some(next_position)
                        }),
                    },
                )
                .collect()
        }
    }

    fn dfs_longest_path(
        start: (usize, usize),
        successor_lut: &SuccessorLUT,
        mut success: impl FnMut(&(usize, usize)) -> bool,
    ) -> Option<usize> {
        fn step(
            position: (usize, usize),
            successor_lut: &SuccessorLUT,
            visited: &mut std::collections::HashSet<(usize, usize)>,
            success: &mut impl FnMut(&(usize, usize)) -> bool,
        ) -> Option<usize> {
            if success(&position) {
                Some(0)
            } else {
                let mut max_distance = None;
                for (&n, offset) in successor_lut.get(&position).unwrap() {
                    if visited.insert(n) {
                        if let Some(distance) = step(n, successor_lut, visited, success) {
                            max_distance = Some(max_distance.unwrap_or(0).max(distance + offset));
                        }
                        visited.remove(&n);
                    }
                }
                max_distance
            }
        }
        step(
            start,
            successor_lut,
            &mut std::collections::HashSet::new(),
            &mut success,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        #.#####################
        #.......#########...###
        #######.#########.#.###
        ###.....#.>.>.###.#.###
        ###v#####.#v#.###.#.###
        ###.>...#.#.#.....#...#
        ###v###.#.#.#########.#
        ###...#.#.#.......#...#
        #####.#.#.#######.#.###
        #.....#.#.#.......#...#
        #.#####.#.#.#########v#
        #.#...#...#...###...>.#
        #.#.#v#######v###.###v#
        #...#.>.#...>.>.#.###.#
        #####v#.#.###v#.#.###.#
        #.....#...#...#.#.#...#
        #.#########.###.#.#.###
        #...###...#...#...#.###
        ###.###.#.###v#####v###
        #...#...#.#.>.>.#.>.###
        #.###.###.#.###.#.#v###
        #.....###...###...#...#
        #####################.#
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SAMPLE)), 94);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SAMPLE)), 154);
    }
}
