use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day16)]
fn parse(input: &str) -> utils::Cave {
    let mut matrix = pathfinding::matrix::Matrix::new(
        input.lines().count(),
        input.lines().next().unwrap().len(),
        utils::Tile::Empty,
    );
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            matrix[(x, y)] = c.into();
        });
    });
    utils::Cave::new(matrix)
}

#[aoc(day16, part1)]
fn part1(input: &utils::Cave) -> usize {
    let beam = utils::Beam::new((0, 0), utils::Direction::East);
    input.get_energized_tiles(beam).len()
}

#[aoc(day16, part2)]
fn part2(input: &utils::Cave) -> usize {
    use rayon::prelude::*;

    (0..input.columns)
        .map(|x| utils::Beam::new((x, 0), utils::Direction::South))
        .chain(
            (0..input.columns)
                .map(|x| utils::Beam::new((x, input.rows - 1), utils::Direction::North)),
        )
        .chain((0..input.rows).map(|y| utils::Beam::new((0, y), utils::Direction::East)))
        .chain(
            (0..input.rows)
                .map(|y| utils::Beam::new((input.columns - 1, y), utils::Direction::West)),
        )
        .par_bridge()
        .map(|beam| input.get_energized_tiles(beam).len())
        .max()
        .unwrap()
}

mod utils {
    #[derive(derive_more::Deref)]
    pub struct Cave(pathfinding::matrix::Matrix<Tile>);

    impl Cave {
        pub fn new(value: pathfinding::matrix::Matrix<Tile>) -> Self {
            Self(value)
        }

        pub fn get_energized_tiles(&self, beam: Beam) -> rustc_hash::FxHashSet<(usize, usize)> {
            self.beamform(beam)
                .iter()
                .map(|beam| beam.position)
                .collect()
        }

        fn beamform(&self, beam: Beam) -> rustc_hash::FxHashSet<Beam> {
            let mut active_beams = vec![beam];
            let mut beams = rustc_hash::FxHashSet::default();
            while let Some(beam) = active_beams.pop() {
                if beams.insert(beam) {
                    active_beams.extend(beam.propagate(self));
                }
            }
            beams
        }
    }

    #[repr(u8)]
    #[derive(Clone, Copy)]
    pub enum Tile {
        Empty = b'.',
        MirrorDiagonal = b'\\',
        MirrorAntiDiagonal = b'/',
        SplitterVertical = b'|',
        SplitterHorizontal = b'-',
    }

    impl From<char> for Tile {
        fn from(c: char) -> Self {
            match c {
                '.' => Self::Empty,
                '\\' => Self::MirrorDiagonal,
                '/' => Self::MirrorAntiDiagonal,
                '|' => Self::SplitterVertical,
                '-' => Self::SplitterHorizontal,
                _ => panic!("Invalid tile"),
            }
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Direction {
        North,
        West,
        South,
        East,
    }

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Beam {
        position: (usize, usize),
        direction: Direction,
    }

    impl Beam {
        pub fn new(position: (usize, usize), direction: Direction) -> Self {
            Self {
                position,
                direction,
            }
        }

        fn propagate(&self, cave: &Cave) -> Vec<Self> {
            match (self.direction, cave[self.position]) {
                (Direction::West, Tile::MirrorDiagonal)
                | (Direction::East, Tile::MirrorAntiDiagonal) => vec![Direction::North],
                (Direction::North, Tile::MirrorDiagonal)
                | (Direction::South, Tile::MirrorAntiDiagonal) => vec![Direction::West],
                (Direction::North, Tile::MirrorAntiDiagonal)
                | (Direction::South, Tile::MirrorDiagonal) => vec![Direction::East],
                (Direction::West, Tile::MirrorAntiDiagonal)
                | (Direction::East, Tile::MirrorDiagonal) => vec![Direction::South],
                (Direction::North | Direction::South, Tile::SplitterHorizontal) => {
                    vec![Direction::West, Direction::East]
                }
                (Direction::West | Direction::East, Tile::SplitterVertical) => {
                    vec![Direction::North, Direction::South]
                }
                (direction, _) => vec![direction],
            }
            .iter()
            .filter_map(|&direction| {
                match direction {
                    Direction::North => self
                        .position
                        .1
                        .checked_sub(1)
                        .map(|next_y| (self.position.0, next_y)),
                    Direction::West => self
                        .position
                        .0
                        .checked_sub(1)
                        .map(|next_x| (next_x, self.position.1)),
                    Direction::South => {
                        let next_y = self.position.1 + 1;
                        (next_y < cave.rows).then_some((self.position.0, next_y))
                    }
                    Direction::East => {
                        let next_x = self.position.0 + 1;
                        (next_x < cave.columns).then_some((next_x, self.position.1))
                    }
                }
                .map(|position| Self::new(position, direction))
            })
            .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {r"
        .|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SAMPLE)), 46);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SAMPLE)), 51);
    }
}
