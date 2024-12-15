use aoc_runner_derive::aoc;

#[aoc(day15, part1)]
#[must_use]
pub fn part1(input: &str) -> usize {
    let mut warehouse = Warehouse::parse(input);
    let steps =
        Warehouse::translate_instructions(warehouse.instructions, warehouse.map.ncols() as isize);
    for step in steps {
        let current_position = warehouse.robot_pos;
        let next_position = current_position.wrapping_add_signed(step);
        match warehouse.map[next_position] {
            b'.' => {
                warehouse.robot_pos = next_position;
            }
            b'O' => {
                if let Some(free_position) =
                    Warehouse::find_next_free_position(warehouse.map.as_ref(), next_position, step)
                {
                    warehouse.robot_pos = next_position;
                    warehouse.map[next_position] = b'.';
                    warehouse.map[free_position] = b'O';
                }
            }
            _ => {}
        }
    }
    Warehouse::score(warehouse.map.as_ref(), warehouse.map.ncols(), b'O')
}

#[aoc(day15, part2)]
#[must_use]
pub fn part2(input: &str) -> usize {
    let warehouse = Warehouse::parse(input);
    let mut map = warehouse
        .map
        .iter()
        .flat_map(|&b| match b {
            b'#' => [b'#', b'#'],
            b'.' => [b'.', b'.'],
            b'O' => [b'[', b']'],
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();
    let mut robot_pos = 2 * (warehouse.map.ncols() * (warehouse.robot_pos / warehouse.map.ncols()))
        + 2 * (warehouse.robot_pos % warehouse.map.ncols());
    let steps = Warehouse::translate_instructions(
        warehouse.instructions,
        2 * warehouse.map.ncols() as isize,
    );
    for step in steps {
        let next_position = robot_pos.wrapping_add_signed(step);
        match map[next_position] {
            b'.' => {
                robot_pos = next_position;
            }
            b'[' | b']' => {
                if Warehouse::check_obstacle(&map, next_position, step) {
                    Warehouse::move_box(&mut map, next_position, step);
                    robot_pos = next_position;
                }
            }
            _ => {}
        }
    }
    Warehouse::score(&map, 2 * warehouse.map.ncols(), b'[')
}

struct Warehouse<'a> {
    map: nalgebra::DMatrix<u8>,
    instructions: &'a str,
    robot_pos: usize,
}

impl<'a> Warehouse<'a> {
    fn parse(input: &'a str) -> Self {
        let (input_map, instructions) = input.split_once("\n\n").unwrap();
        let input_map = input_map.as_bytes();
        let width = input_map.iter().position(|&c| c == b'\n').unwrap();
        let mut map = nalgebra::DMatrix::from_element(width, (input_map.len() - 1) / width, b'.');
        let mut robot_pos = 0;
        input_map
            .chunks(width + 1)
            .enumerate()
            .for_each(|(y, line)| {
                line.iter()
                    .enumerate()
                    .take(width)
                    .for_each(|(x, &c)| match c {
                        b'@' => {
                            robot_pos = x + y * width;
                        }
                        _ => map[(x, y)] = c,
                    });
            });
        Self {
            map,
            instructions,
            robot_pos,
        }
    }

    #[inline]
    fn translate_instructions(
        instructions: &str,
        width: isize,
    ) -> impl Iterator<Item = isize> + '_ {
        instructions
            .bytes()
            .filter(|b| !b.is_ascii_whitespace())
            .map(move |c| match c {
                b'^' => -width,
                b'v' => width,
                b'<' => -1,
                b'>' => 1,
                _ => {
                    unreachable!()
                }
            })
    }

    fn find_next_free_position(map: &[u8], start: usize, step: isize) -> Option<usize> {
        let mut position = start.wrapping_add_signed(step);
        while map[position] == b'O' {
            position = position.wrapping_add_signed(step);
        }
        if map[position] == b'.' {
            Some(position)
        } else {
            None
        }
    }

    #[inline]
    fn score(map: &[u8], cols: usize, box_byte: u8) -> usize {
        map.iter()
            .enumerate()
            .map(|(i, &b)| {
                if b == box_byte {
                    100 * (i / cols) + (i % cols)
                } else {
                    0
                }
            })
            .sum()
    }

    fn check_obstacle(map: &[u8], pos: usize, step: isize) -> bool {
        let (left, right) = match map[pos] {
            b'[' => (pos, pos + 1),
            b']' => (pos - 1, pos),
            b'.' => return true,
            b'#' => return false,
            _ => unreachable!(),
        };

        if step.abs() == 1 {
            Self::check_obstacle(map, pos.wrapping_add_signed(step), step)
        } else if map[left.wrapping_add_signed(step)] == b'[' {
            Self::check_obstacle(map, left.wrapping_add_signed(step), step)
        } else {
            Self::check_obstacle(map, left.wrapping_add_signed(step), step)
                && Self::check_obstacle(map, right.wrapping_add_signed(step), step)
        }
    }

    fn move_box(map: &mut [u8], pos: usize, step: isize) {
        let (left, right) = match map[pos] {
            b'[' => (pos, pos + 1),
            b']' => (pos - 1, pos),
            b'.' => return,
            _ => unreachable!(),
        };

        if step.abs() == 1 {
            Self::move_box(map, pos.wrapping_add_signed(step * 2), step);
        } else {
            Self::move_box(map, left.wrapping_add_signed(step), step);
            Self::move_box(map, right.wrapping_add_signed(step), step);
        }

        map[left] = b'.';
        map[right] = b'.';
        map[left.wrapping_add_signed(step)] = b'[';
        map[right.wrapping_add_signed(step)] = b']';
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE1: &str = indoc! {"
        ##########
        #..O..O.O#
        #......O.#
        #.OO..O.O#
        #..O@..O.#
        #O#..O...#
        #O..O..O.#
        #.OO.O.OO#
        #....O...#
        ##########

        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    "};

    const SAMPLE2: &str = indoc! {"
        ########
        #..O.O.#
        ##@.O..#
        #...O..#
        #.#.O..#
        #...O..#
        #......#
        ########

        <^^>>>vv<v>>v<<
    "};

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(SAMPLE1), 10092);
        assert_eq!(part1(SAMPLE2), 2028);
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(part2(SAMPLE1), 9021);
    }
}
