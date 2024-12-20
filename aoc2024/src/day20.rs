use aoc_runner_derive::aoc;
use strum::IntoEnumIterator;

#[aoc(day20, part1)]
#[must_use]
pub fn part1(input: &str) -> usize {
    Maze::<139>::parse(input).solve1::<100>()
}

#[aoc(day20, part2)]
#[must_use]
pub fn part2(input: &str) -> usize {
    Maze::<139>::parse(input).solve2::<100>()
}

struct Maze<const D: usize> {
    map: nalgebra::SMatrix<u8, D, D>,
    costmap: nalgebra::SMatrix<[usize; 2], D, D>,
    original_cost: usize,
}

impl<const D: usize> Maze<D> {
    #[inline]
    fn parse(input: &str) -> Self {
        let input = input.as_bytes();
        let map = nalgebra::SMatrix::<u8, D, D>::from_iterator(
            input
                .chunks(unsafe { D.unchecked_add(3) })
                .skip(1)
                .take(D)
                .flat_map(|line| line.iter().skip(1).take(D).copied()),
        );
        let pos: Vec<_> = map
            .iter()
            .enumerate()
            .filter(|(_, &b)| b == b'S' || b == b'E')
            .take(2)
            .map(|(i, _)| (i % D, i / D))
            .collect();

        let mut costmap = nalgebra::SMatrix::from_element([usize::MAX; 2]);
        for (i, &pos) in pos.iter().enumerate() {
            costmap[pos][i] = 0;
            let mut queue = pos;
            'outer: loop {
                let new_cost = unsafe { costmap[queue][i].unchecked_add(1) };
                if let Some(new_pos) = Direction::iter()
                    .map(|dir| dir.step(queue))
                    .filter(|&new_pos| new_pos.0 < D && new_pos.1 < D)
                    .filter(|&new_pos| map[new_pos] != b'#')
                    .find(|&new_pos| new_cost < costmap[new_pos][i])
                {
                    queue = new_pos;
                    costmap[new_pos][i] = new_cost;
                    continue 'outer;
                }
                break;
            }
        }

        Self {
            map,
            costmap,
            original_cost: costmap[pos[0]][1],
        }
    }

    #[inline]
    fn solve1<const L: usize>(self) -> usize {
        self.map
            .column_iter()
            .enumerate()
            .flat_map(|(y, col)| {
                col.into_iter()
                    .enumerate()
                    .filter(|(_, &b)| b == b'#')
                    .map(move |(x, _)| (x, y))
            })
            .map(|pos| {
                Direction::iter()
                    .filter(|dir| {
                        let new_pos0 = dir.step(pos);
                        if new_pos0.0 >= D || new_pos0.1 >= D {
                            return false;
                        }
                        let new_cost0 = self.costmap[new_pos0][0];
                        if new_cost0 == usize::MAX {
                            return false;
                        }
                        let new_pos1 = dir.reverse().step(pos);
                        if new_pos1.0 >= D || new_pos1.1 >= D {
                            return false;
                        }
                        let new_cost1 = self.costmap[new_pos1][1];
                        if new_cost1 == usize::MAX {
                            return false;
                        }
                        let new_cost =
                            unsafe { new_cost0.unchecked_add(new_cost1).unchecked_add(2) };
                        new_cost < self.original_cost
                            && unsafe { self.original_cost.unchecked_sub(new_cost) } >= L
                    })
                    .count()
            })
            .sum()
    }

    #[inline]
    fn solve2<const L: usize>(self) -> usize {
        self.map
            .column_iter()
            .enumerate()
            .flat_map(|(y, col)| {
                col.into_iter()
                    .enumerate()
                    .filter(|(_, &b)| b != b'#')
                    .map(move |(x, _)| (x, y))
            })
            .map(|pos| {
                let start_cost = self.costmap[pos][0];
                let pos = (pos.0 as isize, pos.1 as isize);
                (-((pos.0).min(20))..=20)
                    .map(|dx| (dx, unsafe { pos.0.unchecked_add(dx) }))
                    .filter(|&(_, new_x)| new_x < D as isize)
                    .map(|(dx, new_x)| {
                        let dx_abs = dx.abs();
                        let max_y = unsafe { 20_isize.unchecked_sub(dx_abs) };
                        (-((pos.1).min(max_y))..=max_y)
                            .map(|dy| (dy, unsafe { pos.1.unchecked_add(dy) }))
                            .filter(|&(_, new_y)| new_y < D as isize)
                            .filter(|&(dy, new_y)| {
                                let new_pos = (new_x as usize, new_y as usize);
                                let goal_cost = self.costmap[new_pos][1];
                                if goal_cost == usize::MAX {
                                    return false;
                                }
                                let new_cost = unsafe {
                                    start_cost
                                        .unchecked_add(goal_cost)
                                        .unchecked_add(dx_abs.unchecked_add(dy.abs()) as usize)
                                };
                                new_cost < self.original_cost
                                    && unsafe { self.original_cost.unchecked_sub(new_cost) } >= L
                            })
                            .count()
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

#[derive(Clone, Copy, strum::EnumIter)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn reverse(self) -> Self {
        match self {
            Self::N => Self::S,
            Self::E => Self::W,
            Self::S => Self::N,
            Self::W => Self::E,
        }
    }

    fn step(self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Self::N => (pos.0, pos.1.wrapping_sub(1)),
            Self::E => (unsafe { pos.0.unchecked_add(1) }, pos.1),
            Self::S => (pos.0, unsafe { pos.1.unchecked_add(1) }),
            Self::W => (pos.0.wrapping_sub(1), pos.1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        ###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############
    "};

    #[test]
    pub fn part1_example() {
        assert_eq!(Maze::<13>::parse(SAMPLE).solve1::<64>(), 1);
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(Maze::<13>::parse(SAMPLE).solve2::<76>(), 3);
    }
}
