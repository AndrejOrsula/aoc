use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day8, part1)]
#[must_use]
pub fn part1(input: &str) -> usize {
    let map = Map::parse(input);
    map.nodes()
        .iter()
        .filter(|node| !node.is_empty())
        .flat_map(|node| {
            node.iter()
                .permutations(2)
                .map(|pair| (pair[0], pair[1]))
                .filter_map(|(&(x1, y1), &(x2, y2))| {
                    if let (Some(x), Some(y)) = ((2 * x2).checked_sub(x1), (2 * y2).checked_sub(y1))
                    {
                        if x < map.ncols() && y < map.nrows() {
                            return Some((x, y));
                        }
                    }
                    None
                })
        })
        .collect::<rustc_hash::FxHashSet<_>>()
        .len()
}

#[aoc(day8, part2)]
#[must_use]
pub fn part2(input: &str) -> usize {
    let map = Map::parse(input);
    let (ncols, nrows) = (map.ncols() as i16, map.nrows() as i16);
    map.nodes()
        .iter()
        .filter(|node| !node.is_empty())
        .flat_map(|node| {
            node.iter()
                .permutations(2)
                .map(|pair| (pair[0], pair[1]))
                .flat_map(|(&(x1, y1), &(x2, y2))| {
                    let (x1, y1, x, y) = (x1 as i16, y1 as i16, x2 as i16, y2 as i16);
                    let (dx, dy) = ((x - x1), (y - y1));
                    std::iter::successors(Some((x, y)), move |(x, y)| {
                        let (x, y) = (x + dx, y + dy);
                        if x >= 0 && y >= 0 && x < ncols && y < nrows {
                            return Some((x, y));
                        }
                        None
                    })
                })
        })
        .collect::<rustc_hash::FxHashSet<_>>()
        .len()
}

#[repr(transparent)]
#[derive(Clone, derive_more::Deref, derive_more::DerefMut)]
struct Map(nalgebra::DMatrix<u8>);

impl Map {
    fn parse(input: &str) -> Self {
        let input = input.as_bytes();
        let width = input.iter().position(|&c| c == b'\n').unwrap();
        let mut map = nalgebra::DMatrix::from_element(width, input.len() / width, b'.');
        input.chunks(width + 1).enumerate().for_each(|(y, line)| {
            line.iter().enumerate().for_each(|(x, c)| match c {
                b'.' | b'\n' => {}
                _ => map[(x, y)] = *c,
            });
        });
        Self(map)
    }

    fn nodes(&self) -> [smallvec::SmallVec<[(usize, usize); 4]>; u8::MAX as usize] {
        let mut nodes = std::array::from_fn(|_| smallvec::SmallVec::new());
        (0..self.ncols())
            .cartesian_product(0..self.nrows())
            .filter(|&(y, x)| self[(x, y)] != b'.')
            .for_each(|(y, x)| {
                nodes[self[(x, y)] as usize].push((x, y));
            });
        nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
    "};

    const SAMPLE2: &str = indoc! {"
        T.........
        ...T......
        .T........
        ..........
        ..........
        ..........
        ..........
        ..........
        ..........
        ..........
    "};

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(SAMPLE), 14);
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(part2(SAMPLE), 34);
        assert_eq!(part2(SAMPLE2), 9);
    }
}
