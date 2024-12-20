use aoc_runner_derive::aoc;

#[aoc(day18, part1)]
#[must_use]
pub fn part1(input: &str) -> usize {
    Memory::<71>::solve1(input)
}

#[aoc(day18, part2)]
#[must_use]
pub fn part2(input: &str) -> String {
    Memory::<71>::solve2(input)
}

#[repr(transparent)]
#[derive(derive_more::Deref, derive_more::DerefMut)]
struct Memory<const D: usize>(nalgebra::SMatrix<u16, D, D>);

impl<const D: usize> Memory<D> {
    #[inline]
    fn solve1(input: &str) -> usize {
        Memory::<D>::parse(input).search(1024).unwrap()
    }

    #[inline]
    fn solve2(input: &str) -> String {
        let mut mem = Memory::<D>::parse(input);
        let (mut min, mut max) = (0, 2 << 12);
        while min < max {
            let cost = (min + max) / 2;
            if mem.search(cost).is_some() {
                min = unsafe { cost.unchecked_add(1) };
            } else {
                max = cost;
            }
        }
        mem.iter()
            .position(|&time| time == min)
            .map(|i| {
                let (x, y) = (i % D, i / D);
                format!("{x},{y}")
            })
            .unwrap()
    }

    #[inline]
    fn parse(input: &str) -> Self {
        let mut mem = nalgebra::SMatrix::from_element(u16::MAX);
        input
            .split('\n')
            .filter(|s| !s.is_empty())
            .enumerate()
            .for_each(|(i, coordinates)| unsafe {
                let (x, y) = coordinates.split_once(',').unwrap_unchecked();
                let (x, y) = (x.parse().unwrap_unchecked(), y.parse().unwrap_unchecked());

                mem[(x, y)] = i as u16;
            });
        Self(mem)
    }

    #[inline]
    fn search(&mut self, n_bytes: u16) -> Option<usize> {
        let mut seen = **self;
        seen[(0, 0)] = 0;
        let mut queue = std::collections::VecDeque::with_capacity(52);
        queue.push_back(((0, 0), 0));
        while let Some((pos, cost)) = queue.pop_front() {
            if pos == (D - 1, D - 1) {
                return Some(cost);
            }

            [(0, 1), (1, 0), (0, -1), (-1, 0)]
                .into_iter()
                .filter_map(|(dx, dy)| {
                    let new_x = pos.0.wrapping_add_signed(dx);
                    (new_x < D)
                        .then(|| {
                            let new_y = pos.1.wrapping_add_signed(dy);
                            (new_y < D).then_some((new_x, new_y))
                        })
                        .flatten()
                })
                .for_each(|new_pos| {
                    if seen[new_pos] > n_bytes {
                        queue.push_back((new_pos, unsafe { cost.unchecked_add(1) }));
                        seen[new_pos] = 0;
                    }
                });
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0
    "};

    #[test]
    pub fn part1_example() {
        assert_eq!(Memory::<9>::solve1(SAMPLE), 22);
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(Memory::<7>::solve2(SAMPLE), "6,1");
    }
}
