use aoc_runner_derive::aoc;
use itertools::Itertools;
use strum::IntoEnumIterator;

#[aoc(day12, part1)]
#[must_use]
pub fn part1(input: &str) -> usize {
    let mut map = Map::parse(input);
    let mut seen_pos = std::collections::HashSet::default();
    (0..map.ncols())
        .cartesian_product(0..map.nrows())
        .filter_map(|pos| map.flood(&mut seen_pos, pos))
        .map(|(area, perim)| area.len() * perim)
        .sum()
}

#[aoc(day12, part2)]
#[must_use]
pub fn part2(input: &str) -> usize {
    let mut map = Map::parse(input);
    let mut seen_pos = std::collections::HashSet::default();
    (0..map.ncols())
        .cartesian_product(0..map.nrows())
        .filter_map(|x| map.flood(&mut seen_pos, x))
        .map(|(area, _)| {
            let mut n_corners = 0;
            for &point in &area {
                for a in Direction::iter() {
                    let b = a.right();
                    n_corners += usize::from(
                        area.contains(&a.next(point))
                            && area.contains(&b.next(point))
                            && !area.contains(&b.next(a.next(point))),
                    );
                }
                for a in Direction::iter() {
                    n_corners += usize::from(
                        !area.contains(&a.next(point)) && !area.contains(&a.right().next(point)),
                    );
                }
            }
            n_corners * area.len()
        })
        .sum()
}

#[repr(transparent)]
#[derive(Clone, derive_more::Deref, derive_more::DerefMut)]
struct Map(nalgebra::DMatrix<u8>);

impl Map {
    fn parse(input: &str) -> Self {
        let input = input.as_bytes();
        let width = input.iter().position(|&c| c == b'\n').unwrap();
        let mut map = nalgebra::DMatrix::from_element(width, (input.len() - 1) / width, b'0');
        input.chunks(width + 1).enumerate().for_each(|(y, line)| {
            line.iter().enumerate().take(width).for_each(|(x, &c)| {
                map[(x, y)] = c;
            });
        });
        Self(map)
    }

    fn flood(
        &mut self,
        seen_pos: &mut rustc_hash::FxHashSet<(usize, usize)>,
        start: (usize, usize),
    ) -> Option<(rustc_hash::FxHashSet<(usize, usize)>, usize)> {
        if !seen_pos.insert(start) {
            return None;
        }

        let mut area = rustc_hash::FxHashSet::default();
        area.insert(start);
        let mut queue = Vec::new();
        queue.push(start);
        let plant = self.get(start).unwrap();
        let mut perim = 0;
        while let Some(pos) = queue.pop() {
            for next in Direction::iter().map(|x| x.next(pos)) {
                if next.0 >= self.ncols() || next.1 >= self.nrows() {
                    perim += 1;
                    continue;
                }
                if self.get(next).unwrap() == plant {
                    if seen_pos.insert(next) {
                        area.insert(next);
                        queue.push(next);
                    }
                    continue;
                }
                perim += 1;
            }
        }

        Some((area, perim))
    }
}

#[derive(Clone, Copy, strum::EnumIter)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    #[must_use]
    fn next(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Self::Up => (pos.0, pos.1.wrapping_sub(1)),
            Self::Down => (pos.0, pos.1 + 1),
            Self::Left => (pos.0.wrapping_sub(1), pos.1),
            Self::Right => (pos.0 + 1, pos.1),
        }
    }

    #[must_use]
    fn right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE1: &str = indoc! {"
        AAAA
        BBCD
        BBCC
        EEEC
    "};

    const SAMPLE2: &str = indoc! {"
        OOOOO
        OXOXO
        OOOOO
        OXOXO
        OOOOO
    "};

    const SAMPLE3: &str = indoc! {"
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE
    "};

    const SAMPLE4: &str = indoc! {"
        EEEEE
        EXXXX
        EEEEE
        EXXXX
        EEEEE
    "};

    const SAMPLE5: &str = indoc! {"
        AAAAAA
        AAABBA
        AAABBA
        ABBAAA
        ABBAAA
        AAAAAA
    "};

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(SAMPLE1), 140);
        assert_eq!(part1(SAMPLE2), 772);
        assert_eq!(part1(SAMPLE3), 1930);
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(part2(SAMPLE1), 80);
        assert_eq!(part2(SAMPLE2), 436);
        assert_eq!(part2(SAMPLE3), 1206);
        assert_eq!(part2(SAMPLE4), 236);
        assert_eq!(part2(SAMPLE5), 368);
    }
}
