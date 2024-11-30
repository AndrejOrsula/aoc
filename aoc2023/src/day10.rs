use aoc_runner_derive::aoc;

fn parse(input: &str) -> utils::Map {
    let mut grid = pathfinding::matrix::Matrix::new(
        input.lines().count(),
        input.lines().next().unwrap().len(),
        utils::Tile::Ground,
    );
    input.lines().enumerate().for_each(|(x, line)| {
        line.chars().enumerate().for_each(|(y, c)| {
            grid[(x, y)] = c.try_into().unwrap();
        });
    });

    let start_position = grid
        .items()
        .find_map(|((x, y), &tile)| (tile == utils::Tile::Start).then_some((x, y)))
        .unwrap();

    utils::Map {
        grid,
        start_position,
    }
}

#[aoc(day10, part1)]
#[must_use]
pub fn part1(input: &str) -> usize {
    let input = parse(input);
    // The furthest point from the start is always the middle of the loop
    input.get_loop().len() / 2
}

#[aoc(day10, part2)]
#[must_use]
pub fn part2(input: &str) -> usize {
    let input = parse(input);
    // Construct a grid that only contains the loop
    let mut enclosure =
        pathfinding::matrix::Matrix::new(input.grid.rows, input.grid.columns, utils::Tile::Ground);
    for position in input.get_loop() {
        enclosure[position] = input.grid[position];
    }

    // Iterate over the rows of the enclosure, while skipping the first and last rows (not inside the enclosure)
    enclosure
        .iter()
        .skip(1)
        .take(enclosure.rows - 2)
        .map(|row| {
            row.iter()
                .fold((0, false), |(mut n_inside, mut is_inside), &tile| {
                    // Detect the start and discontinuities of the enclosure
                    if let utils::Tile::PipeNorthSouth
                    | utils::Tile::BendNorthWest
                    | utils::Tile::BendNorthEast = tile
                    {
                        is_inside = !is_inside;
                    }

                    // Count only the ground tiles inside the enclosure
                    if is_inside && tile == utils::Tile::Ground {
                        n_inside += 1;
                    }

                    (n_inside, is_inside)
                })
                .0
        })
        .sum()
}

mod utils {
    #[derive(Clone, Copy, PartialEq)]
    pub enum Tile {
        PipeNorthSouth,
        PipeWestEast,
        BendNorthEast,
        BendNorthWest,
        BendSouthWest,
        BendSouthEast,
        Ground,
        Start,
    }

    impl TryFrom<char> for Tile {
        type Error = &'static str;
        fn try_from(c: char) -> Result<Self, Self::Error> {
            match c {
                '|' => Ok(Self::PipeNorthSouth),
                '-' => Ok(Self::PipeWestEast),
                'L' => Ok(Self::BendNorthEast),
                'J' => Ok(Self::BendNorthWest),
                '7' => Ok(Self::BendSouthWest),
                'F' => Ok(Self::BendSouthEast),
                '.' => Ok(Self::Ground),
                'S' => Ok(Self::Start),
                _ => Err("Unknown tile type!"),
            }
        }
    }

    pub struct Map {
        pub grid: pathfinding::matrix::Matrix<Tile>,
        pub start_position: (usize, usize),
    }

    impl Map {
        pub fn get_loop(&self) -> Vec<(usize, usize)> {
            let mut position = self.start_position;
            let mut direction = self.get_initial_direction();
            let mut loop_indices = Vec::new();
            loop {
                // Move in the direction of the pipe
                position = self.grid.move_in_direction(position, direction).unwrap();

                // Add the position to the loop indices
                loop_indices.push(position);

                // Return once the loop is closed
                if position == self.start_position {
                    return loop_indices;
                }

                // Determine the next direction
                direction = match (direction, self.grid[position]) {
                    ((-1 | 1, 0), Tile::PipeNorthSouth) | ((0, -1 | 1), Tile::PipeWestEast) => {
                        direction
                    }
                    ((1, 0), Tile::BendNorthWest) | ((-1, 0), Tile::BendSouthWest) => (0, -1),
                    ((1, 0), Tile::BendNorthEast) | ((-1, 0), Tile::BendSouthEast) => (0, 1),
                    ((0, -1), Tile::BendNorthEast) | ((0, 1), Tile::BendNorthWest) => (-1, 0),
                    ((0, -1), Tile::BendSouthEast) | ((0, 1), Tile::BendSouthWest) => (1, 0),
                    _ => unreachable!("All possible directions are covered!"),
                };
            }
        }

        fn get_initial_direction(&self) -> (isize, isize) {
            let (x, y) = self.start_position;
            if let Some(x_minus_1) = x.checked_sub(1) {
                if let Tile::PipeNorthSouth | Tile::BendSouthWest | Tile::BendSouthEast =
                    self.grid[(x_minus_1, y)]
                {
                    return (-1, 0);
                }
            }
            if x + 1 < self.grid.rows {
                if let Tile::PipeNorthSouth | Tile::BendNorthWest | Tile::BendNorthEast =
                    self.grid[(x + 1, y)]
                {
                    return (1, 0);
                }
            }
            if let Some(y_minus_1) = y.checked_sub(1) {
                if let Tile::PipeWestEast | Tile::BendNorthEast | Tile::BendSouthEast =
                    self.grid[(x, y_minus_1)]
                {
                    return (0, -1);
                }
            }
            if y + 1 < self.grid.columns {
                if let Tile::PipeWestEast | Tile::BendNorthWest | Tile::BendSouthWest =
                    self.grid[(x, y + 1)]
                {
                    return (0, 1);
                }
            }
            unreachable!("All possible initial directions are covered!")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    pub fn part1_example() {
        const SAMPLES: [&str; 2] = [
            indoc! {"
                .....
                .S-7.
                .|.|.
                .L-J.
                .....
            "},
            indoc! {"
                ..F7.
                .FJ|.
                SJ.L7
                |F--J
                LJ...
            "},
        ];

        assert_eq!(part1(SAMPLES[0]), 4);
        assert_eq!(part1(SAMPLES[1]), 8);
    }

    #[test]
    pub fn part2_example() {
        const SAMPLES: [&str; 3] = [
            indoc! {"
                ...........
                .S-------7.
                .|F-----7|.
                .||.....||.
                .||.....||.
                .|L-7.F-J|.
                .|..|.|..|.
                .L--J.L--J.
                ...........
            "},
            indoc! {"
                .F----7F7F7F7F-7....
                .|F--7||||||||FJ....
                .||.FJ||||||||L7....
                FJL7L7LJLJ||LJ.L-7..
                L--J.L7...LJS7F-7L7.
                ....F-J..F7FJ|L7L7L7
                ....L7.F7||L7|.L7L7|
                .....|FJLJ|FJ|F7|.LJ
                ....FJL-7.||.||||...
                ....L---J.LJ.LJLJ...
            "},
            indoc! {"
                FF7FSF7F7F7F7F7F---7
                L|LJ||||||||||||F--J
                FL-7LJLJ||||||LJL-77
                F--JF--7||LJLJ7F7FJ-
                L---JF-JLJ.||-FJLJJ7
                |F|F-JF---7F7-L7L|7|
                |FFJF7L7F-JF7|JL---7
                7-L-JL7||F7|L7F-7F7|
                L.L7LFJ|||||FJL7||LJ
                L7JLJL-JLJLJL--JLJ.L
            "},
        ];

        assert_eq!(part2(SAMPLES[0]), 4);
        assert_eq!(part2(SAMPLES[1]), 8);
        assert_eq!(part2(SAMPLES[2]), 10);
    }
}
