use aoc_runner_derive::aoc;
use strum::IntoEnumIterator;

#[aoc(day16, part1)]
#[must_use]
pub fn part1(input: &str) -> usize {
    Maze::parse(input).solve1()
}

#[aoc(day16, part2)]
#[must_use]
pub fn part2(input: &str) -> usize {
    Maze::parse(input).solve2()
}

struct Maze {
    map: nalgebra::DMatrix<u8>,
    start_pos: nalgebra::Vector2<usize>,
    goal_pos: nalgebra::Vector2<usize>,
}

impl Maze {
    fn parse(input: &str) -> Self {
        let input = input.as_bytes();
        let width = input.iter().position(|&c| c == b'\n').unwrap();
        let mut map = nalgebra::DMatrix::from_element(width, (input.len() - 1) / width, b'.');
        let mut start_pos = nalgebra::Vector2::new(0, 0);
        let mut goal_pos = nalgebra::Vector2::new(0, 0);
        input.chunks(width + 1).enumerate().for_each(|(y, line)| {
            line.iter().enumerate().take(width).for_each(|(x, &b)| {
                map[(x, y)] = b;
                if b == b'S' {
                    start_pos = nalgebra::Vector2::new(x, y);
                } else if b == b'E' {
                    goal_pos = nalgebra::Vector2::new(x, y);
                }
            });
        });
        Self {
            map,
            start_pos,
            goal_pos,
        }
    }

    fn solve1(self) -> usize {
        let mut seen = rustc_hash::FxHashSet::default();
        let mut proc = std::collections::BinaryHeap::new();
        proc.push(Step::new(self.start_pos, Direction::W, 0));
        while let Some(Step { pos, dir, cost }) = proc.pop() {
            if !seen.insert((pos, dir)) {
                continue;
            }
            if self.map[(pos.x, pos.y)] == b'E' {
                return cost;
            }
            let new = dir.step(pos);
            if new.x < self.map.ncols()
                && new.y < self.map.nrows()
                && self.map[(new.x, new.y)] != b'#'
            {
                proc.push(Step::new(new, dir, cost.wrapping_add(1)));
            }
            for dir in [dir.left(), dir.right()] {
                proc.push(Step::new(pos, dir, cost.wrapping_add(1000)));
            }
        }
        unreachable!()
    }

    fn solve2(self) -> usize {
        let mut total_cost =
            nalgebra::DMatrix::from_element(self.map.nrows(), self.map.ncols(), [usize::MAX; 4]);
        let mut seen = rustc_hash::FxHashSet::default();
        let mut proc = std::collections::BinaryHeap::new();
        proc.push(Step::new(self.start_pos, Direction::E, 0));
        while let Some(Step { pos, dir, cost }) = proc.pop() {
            let c = &mut total_cost[(pos.x, pos.y)];
            c[dir as usize] = c[dir as usize].min(cost);
            if !seen.insert((pos, dir)) {
                continue;
            }
            if self.map[(pos.x, pos.y)] == b'E' {
                break;
            }
            let new = dir.step(pos);
            if new.x < self.map.ncols()
                && new.y < self.map.nrows()
                && self.map[(new.x, new.y)] != b'#'
            {
                proc.push(Step::new(new, dir, cost.wrapping_add(1)));
            }
            for dir in [dir.left(), dir.right()] {
                proc.push(Step::new(pos, dir, cost.wrapping_add(1000)));
            }
        }

        let mut seen = rustc_hash::FxHashSet::default();
        proc.clear();
        for dir in Direction::iter() {
            proc.push(Step::new(
                self.goal_pos,
                dir,
                total_cost[(self.goal_pos.x, self.goal_pos.y)][dir as usize],
            ));
        }
        while let Some(step) = proc.pop() {
            seen.insert(step.pos);
            if step.pos == self.start_pos {
                continue;
            }
            for new in [
                Step::new(
                    step.dir.reverse().step(step.pos),
                    step.dir,
                    step.cost.wrapping_sub(1),
                ),
                Step::new(step.pos, step.dir.left(), step.cost.saturating_sub(1000)),
                Step::new(step.pos, step.dir.right(), step.cost.saturating_sub(1000)),
            ] {
                if new.pos.x < self.map.ncols()
                    && new.pos.y < self.map.nrows()
                    && self.map[(new.pos.x, new.pos.y)] != b'#'
                    && new.cost == total_cost[(new.pos.x, new.pos.y)][new.dir as usize]
                {
                    total_cost[(new.pos.x, new.pos.y)][new.dir as usize] = usize::MAX;
                    proc.push(new);
                }
            }
        }
        seen.len()
    }
}

#[derive(Eq)]
struct Step {
    pos: nalgebra::Vector2<usize>,
    dir: Direction,
    cost: usize,
}

impl Step {
    fn new(pos: nalgebra::Vector2<usize>, dir: Direction, cost: usize) -> Self {
        Self { pos, dir, cost }
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, strum::EnumIter)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn left(self) -> Self {
        match self {
            Self::N => Self::W,
            Self::E => Self::N,
            Self::S => Self::E,
            Self::W => Self::S,
        }
    }

    fn right(self) -> Self {
        match self {
            Self::N => Self::E,
            Self::E => Self::S,
            Self::S => Self::W,
            Self::W => Self::N,
        }
    }

    fn reverse(self) -> Self {
        match self {
            Self::N => Self::S,
            Self::E => Self::W,
            Self::S => Self::N,
            Self::W => Self::E,
        }
    }

    fn step(self, pos: nalgebra::Vector2<usize>) -> nalgebra::Vector2<usize> {
        match self {
            Self::N => nalgebra::Vector2::new(pos.x, pos.y.wrapping_sub(1)),
            Self::E => nalgebra::Vector2::new(pos.x + 1, pos.y),
            Self::S => nalgebra::Vector2::new(pos.x, pos.y + 1),
            Self::W => nalgebra::Vector2::new(pos.x.wrapping_sub(1), pos.y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE1: &str = indoc! {"
        ###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############
    "};
    const SAMPLE2: &str = indoc! {"
        #################
        #...#...#...#..E#
        #.#.#.#.#.#.#.#.#
        #.#.#.#...#...#.#
        #.#.#.#.###.#.#.#
        #...#.#.#.....#.#
        #.#.#.#.#.#####.#
        #.#...#.#.#.....#
        #.#.#####.#.###.#
        #.#.#.......#...#
        #.#.###.#####.###
        #.#.#...#.....#.#
        #.#.#.#####.###.#
        #.#.#.........#.#
        #.#.#.#########.#
        #S#.............#
        #################
    "};

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(SAMPLE1), 7036);
        assert_eq!(part1(SAMPLE2), 11048);
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(part2(SAMPLE1), 45);
        assert_eq!(part2(SAMPLE2), 64);
    }
}
