use aoc_runner_derive::aoc;
use strum::IntoEnumIterator;

#[aoc(day20, part1)]
#[must_use]
pub fn part1(input: &str) -> usize {
    Maze::<141>::parse(input).solve1::<100>()
}

#[aoc(day20, part2)]
#[must_use]
pub fn part2(input: &str) -> usize {
    Maze::<141>::parse(input).solve2::<100>()
}

struct Maze<const D: usize> {
    map: nalgebra::SMatrix<u8, D, D>,
    start_pos: (usize, usize),
    goal_pos: (usize, usize),
}

impl<const D: usize> Maze<D> {
    fn parse(input: &str) -> Self {
        let input = input.as_bytes();
        let width = unsafe { input.iter().position(|&c| c == b'\n').unwrap_unchecked() };
        let mut map = nalgebra::SMatrix::from_element(b'.');
        let mut start_pos = (0, 0);
        let mut goal_pos = (0, 0);
        input
            .chunks(unsafe { width.unchecked_add(1) })
            .enumerate()
            .for_each(|(y, line)| {
                line.iter()
                    .enumerate()
                    .take(width)
                    .for_each(|(x, &b)| match b {
                        b'S' => start_pos = (x, y),
                        b'E' => goal_pos = (x, y),
                        b'#' => map[(x, y)] = b,
                        _ => {}
                    });
            });

        Self {
            map,
            start_pos,
            goal_pos,
        }
    }

    fn compute_cost(&self, start_pos: (usize, usize)) -> nalgebra::DMatrix<usize> {
        let mut cost =
            nalgebra::DMatrix::from_element(self.map.nrows(), self.map.ncols(), usize::MAX);
        cost[start_pos] = 0;
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(start_pos);
        while let Some(pos) = queue.pop_front() {
            let new_cost = unsafe { cost[pos].unchecked_add(1) };
            Direction::iter()
                .map(|dir| dir.step(pos))
                .filter(|&new_pos| self.map[new_pos] == b'.')
                .for_each(|new_pos| {
                    if new_cost < cost[new_pos] {
                        queue.push_back(new_pos);
                        cost[new_pos] = new_cost;
                    }
                });
        }
        cost
    }

    fn solve1<const L: usize>(self) -> usize {
        let cost_from_start = self.compute_cost(self.start_pos);
        let cost_from_goal = self.compute_cost(self.goal_pos);
        let original_cost = cost_from_start[self.goal_pos];
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
                        let from_start_pos = dir.step(pos);
                        if from_start_pos.0 >= self.map.nrows()
                            || from_start_pos.1 >= self.map.ncols()
                        {
                            return false;
                        }
                        let start_cost = cost_from_start[from_start_pos];
                        if start_cost == usize::MAX {
                            return false;
                        }
                        let from_goal_pos = dir.reverse().step(pos);
                        if from_goal_pos.0 >= self.map.nrows()
                            || from_goal_pos.1 >= self.map.ncols()
                        {
                            return false;
                        }
                        let goal_cost = cost_from_goal[from_goal_pos];
                        if goal_cost == usize::MAX {
                            return false;
                        }
                        let new_cost =
                            unsafe { start_cost.unchecked_add(goal_cost).unchecked_add(2) };
                        new_cost < original_cost
                            && unsafe { original_cost.unchecked_sub(new_cost) } >= L
                    })
                    .count()
            })
            .sum()
    }

    fn solve2<const L: usize>(self) -> usize {
        let cost_from_start = self.compute_cost(self.start_pos);
        let cost_from_goal = self.compute_cost(self.goal_pos);
        let original_cost = cost_from_start[self.goal_pos];
        self.map
            .column_iter()
            .enumerate()
            .flat_map(|(y, col)| {
                col.into_iter()
                    .enumerate()
                    .filter(|(_, &b)| b == b'.')
                    .map(move |(x, _)| (x, y))
            })
            .map(|pos| {
                let start_cost = cost_from_start[pos];
                let pos = (pos.0 as isize, pos.1 as isize);
                (-((pos.0).min(20))..=20)
                    .map(|dx| (dx, unsafe { pos.0.unchecked_add(dx) }))
                    .filter(|&(_, new_x)| new_x < self.map.nrows() as isize)
                    .map(|(dx, new_x)| {
                        let dx_abs = dx.abs();
                        let max_y = unsafe { 20_isize.unchecked_sub(dx_abs) };
                        (-((pos.1).min(max_y))..=max_y)
                            .map(|dy| (dy, unsafe { pos.1.unchecked_add(dy) }))
                            .filter(|&(_, new_y)| new_y < self.map.ncols() as isize)
                            .filter(|&(dy, new_y)| {
                                let new_pos = (new_x as usize, new_y as usize);
                                let goal_cost = cost_from_goal[new_pos];
                                if goal_cost == usize::MAX {
                                    return false;
                                }
                                let new_cost = unsafe {
                                    start_cost
                                        .unchecked_add(goal_cost)
                                        .unchecked_add(dx_abs.unchecked_add(dy.abs()) as usize)
                                };
                                new_cost < original_cost
                                    && unsafe { original_cost.unchecked_sub(new_cost) } >= L
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
        assert_eq!(Maze::<15>::parse(SAMPLE).solve1::<64>(), 1);
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(Maze::<15>::parse(SAMPLE).solve2::<76>(), 3);
    }
}
