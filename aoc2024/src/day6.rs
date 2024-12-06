use aoc_runner_derive::aoc;
use itertools::Itertools;
use rayon::prelude::*;

#[aoc(day6, part1)]
#[must_use]
pub fn part1(input: &str) -> u16 {
    let (map, x, y) = Map::parse(input);
    map.n_visited(x, y)
}

#[aoc(day6, part2)]
#[must_use]
pub fn part2(input: &str) -> u16 {
    let (map, x, y) = Map::parse(input);
    let visited_map = map.visited(x, y);
    (0..map.ncols())
        .cartesian_product(0..map.nrows())
        .par_bridge()
        .filter(|&(y, x)| visited_map[(x, y)])
        .map(|(obstruct_y, obstruct_x)| {
            let mut map = map.clone();
            map[(obstruct_x, obstruct_y)] = true;

            let mut walked_map =
                nalgebra::DMatrix::from_element(map.nrows(), map.ncols(), [false; 4]);
            let (mut x, mut y) = (x, y);
            let mut dir = Direction::North;
            loop {
                if map.step(&mut x, &mut y, &mut dir) {
                    let seen = &mut walked_map[(x, y)][dir as usize];
                    if *seen {
                        break 1;
                    }
                    *seen = true;
                } else {
                    break 0;
                }
            }
        })
        .sum()
}

#[repr(transparent)]
#[derive(Clone, derive_more::Deref, derive_more::DerefMut)]
struct Map(nalgebra::DMatrix<bool>);

impl Map {
    fn parse(input: &str) -> (Self, usize, usize) {
        let input = input.as_bytes();
        let width = input.iter().position(|&c| c == b'\n').unwrap();
        let mut map = nalgebra::DMatrix::from_element(input.len() / width, width, false);
        let (mut init_x, mut init_y) = (0, 0);
        input.chunks(width + 1).enumerate().for_each(|(y, line)| {
            line.iter().enumerate().for_each(|(x, c)| match c {
                b'#' => map[(x, y)] = true,
                b'^' => {
                    init_x = x;
                    init_y = y;
                }
                _ => {}
            });
        });
        (Self(map), init_x, init_y)
    }

    fn step(&self, x: &mut usize, y: &mut usize, dir: &mut Direction) -> bool {
        if let Some((next_x, next_y)) = match dir {
            Direction::North => y.checked_sub(1).map(|y| (*x, y)),
            Direction::South => {
                let y = *y + 1;
                (y < self.ncols()).then_some((*x, y))
            }
            Direction::West => x.checked_sub(1).map(|x| (x, *y)),
            Direction::East => {
                let x = *x + 1;
                (x < self.ncols()).then_some((x, *y))
            }
        } {
            if self[(next_x, next_y)] {
                dir.turn();
            } else {
                *x = next_x;
                *y = next_y;
            }
            true
        } else {
            false
        }
    }

    fn visited(&self, mut x: usize, mut y: usize) -> Self {
        let mut visited = nalgebra::DMatrix::from_element(self.nrows(), self.ncols(), false);
        visited[(x, y)] = true;
        let mut dir = Direction::North;
        loop {
            if !self.step(&mut x, &mut y, &mut dir) {
                break;
            }
            visited[(x, y)] = true;
        }
        Self(visited)
    }

    fn n_visited(&self, mut x: usize, mut y: usize) -> u16 {
        let mut n = 1;
        let mut visited = nalgebra::DMatrix::from_element(self.nrows(), self.ncols(), false);
        visited[(x, y)] = true;
        let mut dir = Direction::North;
        loop {
            if !self.step(&mut x, &mut y, &mut dir) {
                break;
            }
            let seen = &mut visited[(x, y)];
            if !*seen {
                *seen = true;
                n += 1;
            }
        }
        n
    }
}

#[repr(usize)]
#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn(&mut self) {
        match self {
            Direction::North => *self = Direction::East,
            Direction::East => *self = Direction::South,
            Direction::South => *self = Direction::West,
            Direction::West => *self = Direction::North,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    "};

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(SAMPLE), 41);
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(part2(SAMPLE), 6);
    }
}
