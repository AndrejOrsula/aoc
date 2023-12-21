use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day21)]
fn parse(input: &str) -> utils::Map {
    let mut grid =
        pathfinding::grid::Grid::new(input.lines().count(), input.lines().next().unwrap().len());
    let mut start_position = None;

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| match c {
            '#' => {}
            '.' => {
                grid.add_vertex((x, y));
            }
            'S' => {
                grid.add_vertex((x, y));
                assert!(start_position.is_none());
                start_position = Some((x, y));
            }
            _ => panic!("Invalid map tile"),
        });
    });

    utils::Map::new(grid, start_position.unwrap())
}

#[aoc(day21, part1)]
fn part1(input: &utils::Map) -> usize {
    input.count_reachable_plots(64, false)
}

#[aoc(day21, part2)]
fn part2(input: &utils::Map) -> usize {
    input.count_reachable_plots2(26_501_365)
}

mod utils {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Plot {
        pub position: (usize, usize),
        pub layer: (isize, isize),
    }

    impl Plot {
        pub fn new(position: (usize, usize), layer: (isize, isize)) -> Self {
            Self { position, layer }
        }
    }

    pub struct Map {
        grid: pathfinding::grid::Grid,
        start_position: (usize, usize),
    }

    impl Map {
        pub fn new(grid: pathfinding::grid::Grid, start_position: (usize, usize)) -> Self {
            Self {
                grid,
                start_position,
            }
        }

        pub fn reachable_plots(
            &self,
            steps: usize,
            infinite_tiling: bool,
        ) -> rustc_hash::FxHashSet<Plot> {
            let start_plot = Plot::new(self.start_position, (0, 0));

            let mut plots = rustc_hash::FxHashSet::default();
            plots.insert(start_plot);

            for _ in 0..steps {
                let mut next_plots = rustc_hash::FxHashSet::default();
                for plot in plots {
                    self.grid
                        .neighbours(plot.position)
                        .iter()
                        .for_each(|&neighbour| {
                            next_plots.insert(Plot::new(neighbour, plot.layer));
                        });

                    if infinite_tiling {
                        if plot.position.0 == 0 {
                            next_plots.insert(Plot::new(
                                (self.grid.width - 1, plot.position.1),
                                (plot.layer.0 - 1, plot.layer.1),
                            ));
                        }
                        if plot.position.0 == self.grid.width - 1 {
                            next_plots.insert(Plot::new(
                                (0, plot.position.1),
                                (plot.layer.0 + 1, plot.layer.1),
                            ));
                        }
                        if plot.position.1 == 0 {
                            next_plots.insert(Plot::new(
                                (plot.position.0, self.grid.height - 1),
                                (plot.layer.0, plot.layer.1 - 1),
                            ));
                        }
                        if plot.position.1 == self.grid.height - 1 {
                            next_plots.insert(Plot::new(
                                (plot.position.0, 0),
                                (plot.layer.0, plot.layer.1 + 1),
                            ));
                        }
                    }
                }
                plots = next_plots;
            }

            plots
        }

        /// General solution but slow for large steps.
        pub fn count_reachable_plots(&self, steps: usize, infinite_tiling: bool) -> usize {
            self.reachable_plots(steps, infinite_tiling).len()
        }

        /// Non-general solution but fast under the given assumptions.
        ///
        /// # Assumptions
        /// - There are no obstacles in the row and column of the start position
        /// - There are no obstacles at the edges of the map
        ///
        /// # Note
        /// I did not come up with this solution myself but instead found
        /// the description of the approach on r/adventofcode.
        pub fn count_reachable_plots2(&self, steps: usize) -> usize {
            let start_plot = Plot::new(self.start_position, (0, 0));

            let mut plots = rustc_hash::FxHashMap::default();
            plots.insert(start_plot, 0);
            let mut next_plots = std::collections::VecDeque::default();
            next_plots.push_back((start_plot, 0));
            while let Some((plot, distance)) = next_plots.pop_front() {
                self.neighbours(plot, true)
                    .iter()
                    .filter(|&neighbour| {
                        neighbour.layer.0.unsigned_abs() < 2 && neighbour.layer.1.unsigned_abs() < 2
                    })
                    .for_each(|&neighbour| {
                        if let std::collections::hash_map::Entry::Vacant(entry) =
                            plots.entry(neighbour)
                        {
                            let next_distance = distance + 1;
                            entry.insert(next_distance);
                            next_plots.push_back((neighbour, next_distance));
                        }
                    });
            }

            let mut lut = vec![0; steps + 1];
            (0..=steps).rev().for_each(|i| {
                lut[i] = 2 * lut.get(self.grid.height + i).unwrap_or(&0)
                    - lut.get(2 * self.grid.width + i).unwrap_or(&0)
                    + i % 2;
            });
            plots
                .iter()
                .filter(|(_, &distance)| distance <= steps)
                .fold(0, |mut acc, (&plot, &distance)| {
                    let (x_dist, y_dist) = self.distance(start_plot, plot);
                    if -(isize::try_from(self.grid.width).unwrap()) <= x_dist
                        && -(isize::try_from(self.grid.height).unwrap()) <= y_dist
                        && x_dist < isize::try_from(self.grid.width).unwrap()
                        && y_dist < isize::try_from(self.grid.height).unwrap()
                    {
                        acc += lut[distance];
                    }
                    acc
                })
        }

        fn neighbours(&self, plot: Plot, infinite_tiling: bool) -> Vec<Plot> {
            let mut neighbours = Vec::new();

            self.grid
                .neighbours(plot.position)
                .iter()
                .for_each(|&neighbour| {
                    neighbours.push(Plot::new(neighbour, plot.layer));
                });

            if infinite_tiling {
                if plot.position.0 == 0 {
                    neighbours.push(Plot::new(
                        (self.grid.width - 1, plot.position.1),
                        (plot.layer.0 - 1, plot.layer.1),
                    ));
                }
                if plot.position.0 == self.grid.width - 1 {
                    neighbours.push(Plot::new(
                        (0, plot.position.1),
                        (plot.layer.0 + 1, plot.layer.1),
                    ));
                }
                if plot.position.1 == 0 {
                    neighbours.push(Plot::new(
                        (plot.position.0, self.grid.height - 1),
                        (plot.layer.0, plot.layer.1 - 1),
                    ));
                }
                if plot.position.1 == self.grid.height - 1 {
                    neighbours.push(Plot::new(
                        (plot.position.0, 0),
                        (plot.layer.0, plot.layer.1 + 1),
                    ));
                }
            }

            neighbours
        }

        fn distance(&self, plot1: Plot, plot2: Plot) -> (isize, isize) {
            (
                (isize::try_from(plot1.position.0).unwrap()
                    + plot1.layer.0 * isize::try_from(self.grid.width).unwrap())
                    - (isize::try_from(plot2.position.0).unwrap()
                        + plot2.layer.0 * isize::try_from(self.grid.height).unwrap()),
                (isize::try_from(plot1.position.1).unwrap()
                    + plot1.layer.1 * isize::try_from(self.grid.width).unwrap())
                    - (isize::try_from(plot2.position.1).unwrap()
                        + plot2.layer.1 * isize::try_from(self.grid.height).unwrap()),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        ...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........
    "};

    #[test]
    fn part1_example() {
        assert_eq!(parse(SAMPLE).count_reachable_plots(6, false), 16);
    }

    #[test]
    fn part2_example() {
        assert_eq!(parse(SAMPLE).count_reachable_plots(6, true), 16);
        assert_eq!(parse(SAMPLE).count_reachable_plots(10, true), 50);
        assert_eq!(parse(SAMPLE).count_reachable_plots(50, true), 1594);
        assert_eq!(parse(SAMPLE).count_reachable_plots(100, true), 6536);
        // assert_eq!(parse(SAMPLE).count_reachable_plots(500, true), 167_004);
        // assert_eq!(parse(SAMPLE).count_reachable_plots(1000, true), 668_697);
        // assert_eq!(parse(SAMPLE).count_reachable_plots(5000, true), 16_733_044);
    }
}
