use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day17)]
fn parse(input: &str) -> utils::Map {
    let mut matrix = pathfinding::matrix::Matrix::new(
        input.lines().count(),
        input.lines().next().unwrap().len(),
        0,
    );
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            matrix[(x, y)] = c.to_digit(10).unwrap();
        });
    });
    utils::Map::new(matrix)
}

#[aoc(day17, part1)]
fn part1(input: &utils::Map) -> u32 {
    input.find_shortest_path(0, 3)
}

#[aoc(day17, part2)]
fn part2(input: &utils::Map) -> u32 {
    input.find_shortest_path(4, 10)
}

mod utils {
    #[derive(derive_more::Deref)]
    pub struct Map(pathfinding::matrix::Matrix<u32>);

    impl Map {
        pub fn new(value: pathfinding::matrix::Matrix<u32>) -> Self {
            Self(value)
        }

        pub fn find_shortest_path(
            &self,
            min_consecutive_moves: usize,
            max_consecutive_moves: usize,
        ) -> u32 {
            let start_nodes = [
                Node::new((0, 0), Direction::South, 0),
                Node::new((0, 0), Direction::East, 0),
            ];

            start_nodes
                .iter()
                .map(|start| {
                    let (_, distance) = pathfinding::prelude::dijkstra(
                        start,
                        |node| node.successors(self, min_consecutive_moves, max_consecutive_moves),
                        |node| node.success(self, min_consecutive_moves),
                    )
                    .unwrap();
                    distance
                })
                .min()
                .unwrap()
        }

        fn propagate(
            &self,
            position: (usize, usize),
            direction: Direction,
        ) -> Option<(usize, usize)> {
            match direction {
                Direction::North => position.1.checked_sub(1).map(|next_y| (position.0, next_y)),
                Direction::West => position.0.checked_sub(1).map(|next_x| (next_x, position.1)),
                Direction::South => {
                    let next_y = position.1 + 1;
                    (next_y < self.rows).then_some((position.0, next_y))
                }
                Direction::East => {
                    let next_x = position.0 + 1;
                    (next_x < self.columns).then_some((next_x, position.1))
                }
            }
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    enum Direction {
        North,
        West,
        South,
        East,
    }

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    struct Node {
        position: (usize, usize),
        direction: Direction,
        n_consecutive_moves: usize,
    }

    impl Node {
        fn new(position: (usize, usize), direction: Direction, n_consecutive_moves: usize) -> Self {
            Self {
                position,
                direction,
                n_consecutive_moves,
            }
        }

        fn available_directions(
            &self,
            min_consecutive_moves: usize,
            max_consecutive_moves: usize,
        ) -> Vec<Direction> {
            if self.n_consecutive_moves < min_consecutive_moves {
                vec![self.direction]
            } else if self.n_consecutive_moves == max_consecutive_moves {
                match self.direction {
                    Direction::North | Direction::South => {
                        vec![Direction::West, Direction::East]
                    }
                    Direction::West | Direction::East => {
                        vec![Direction::North, Direction::South]
                    }
                }
            } else {
                match self.direction {
                    Direction::North => {
                        vec![Direction::North, Direction::West, Direction::East]
                    }
                    Direction::West => {
                        vec![Direction::West, Direction::North, Direction::South]
                    }
                    Direction::South => {
                        vec![Direction::South, Direction::West, Direction::East]
                    }
                    Direction::East => {
                        vec![Direction::East, Direction::North, Direction::South]
                    }
                }
            }
        }

        fn successors(
            &self,
            map: &Map,
            min_consecutive_moves: usize,
            max_consecutive_moves: usize,
        ) -> Vec<(Node, u32)> {
            self.available_directions(min_consecutive_moves, max_consecutive_moves)
                .iter()
                .filter_map(|&next_direction| {
                    map.propagate(self.position, next_direction)
                        .map(|next_position| {
                            let next_n_consecutive_moves = if next_direction == self.direction {
                                self.n_consecutive_moves + 1
                            } else {
                                1
                            };
                            (
                                Node::new(next_position, next_direction, next_n_consecutive_moves),
                                map[next_position],
                            )
                        })
                })
                .collect()
        }

        fn success(&self, map: &Map, min_consecutive_moves: usize) -> bool {
            (self.position.0 + 1 == map.columns && self.position.1 + 1 == map.rows)
                && self.n_consecutive_moves >= min_consecutive_moves
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SAMPLE)), 102);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SAMPLE)), 94);
    }
}
