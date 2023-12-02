use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<utils::Game> {
    let regex_game_id = regex::Regex::new(r"Game (\d+):").unwrap();
    let regex_game_content = regex::Regex::new(r"Game \d+: (.+)").unwrap();

    input
        .lines()
        .map(|line| {
            // Parse game ID
            let game_id = regex_game_id
                .captures(line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap();

            // Parse game content into cube sets
            let game_content = regex_game_content
                .captures(line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();
            let cube_sets = game_content
                .split(';')
                .map(|cubes| {
                    let mut cube_set = utils::CubeSet::default();
                    cubes.trim().split(',').for_each(|cube| {
                        let mut cube = cube.trim().split(' ');
                        let n = cube.next().unwrap().parse().unwrap();
                        let color = utils::Color::from(cube.next().unwrap());
                        cube_set.set_n_color(color, n);
                    });
                    cube_set
                })
                .collect();

            // Form game from game ID and cube sets
            utils::Game { game_id, cube_sets }
        })
        .collect()
}

mod utils {
    #[derive(Debug)]
    pub struct Game {
        pub game_id: u32,
        pub cube_sets: Vec<CubeSet>,
    }

    #[derive(Debug, Default)]
    pub struct CubeSet {
        pub n_red: u8,
        pub n_green: u8,
        pub n_blue: u8,
    }

    impl CubeSet {
        pub fn set_n_color(&mut self, color: Color, n: u8) {
            match color {
                Color::Red => self.n_red = n,
                Color::Green => self.n_green = n,
                Color::Blue => self.n_blue = n,
            }
        }
    }

    #[derive(Debug)]
    pub enum Color {
        Red,
        Green,
        Blue,
    }

    impl From<&str> for Color {
        fn from(s: &str) -> Self {
            match s {
                "red" => Self::Red,
                "green" => Self::Green,
                "blue" => Self::Blue,
                _ => panic!("Invalid color: {}", s),
            }
        }
    }
}

#[aoc(day2, part1)]
fn part1(input: &[utils::Game]) -> u32 {
    const MAX_N_RED: u8 = 12;
    const MAX_N_GREEN: u8 = 13;
    const MAX_N_BLUE: u8 = 14;

    input
        .iter()
        .filter_map(|game| {
            // Determine if game is valid
            let is_game_valid = game
                .cube_sets
                .iter()
                .find_map(|cube_set| {
                    if cube_set.n_red > MAX_N_RED
                        || cube_set.n_green > MAX_N_GREEN
                        || cube_set.n_blue > MAX_N_BLUE
                    {
                        Some(false)
                    } else {
                        None
                    }
                })
                .unwrap_or(true);

            // Return the ID of the game if valid to be summed
            if is_game_valid {
                Some(game.game_id)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[utils::Game]) -> u32 {
    input
        .iter()
        .map(|game| {
            // Determine the minimum number of cubes of each color
            let mut min_n_red = u8::MIN;
            let mut min_n_green = u8::MIN;
            let mut min_n_blue = u8::MIN;
            game.cube_sets.iter().for_each(|cube_set| {
                min_n_red = min_n_red.max(cube_set.n_red);
                min_n_green = min_n_green.max(cube_set.n_green);
                min_n_blue = min_n_blue.max(cube_set.n_blue);
            });

            // Return the product of the minimum number of cubes of each color
            (min_n_red as u32) * (min_n_green as u32) * (min_n_blue as u32)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SAMPLE)), 8);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SAMPLE)), 2286);
    }
}
