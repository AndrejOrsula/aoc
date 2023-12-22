use aoc_runner_derive::{aoc, aoc_generator};

/// Parsed bricks are already settled on the ground and sorted by their Z coordinate.
#[aoc_generator(day22)]
fn parse(input: &str) -> utils::Brickfall {
    use itertools::Itertools;
    use std::str::FromStr;

    utils::Brickfall::apply_gravity(
        input
            .lines()
            .map(|line| utils::Brick::from_str(line).unwrap())
            .sorted_by(|brick_a, brick_b| brick_a.start.2.cmp(&brick_b.start.2))
            .collect_vec()
            .into(),
    )
}

#[aoc(day22, part1)]
fn part1(input: &utils::Brickfall) -> usize {
    (0..input.len())
        .filter(|&i| input.bricks_affected_by(i).all(|x| !x))
        .count()
}

#[aoc(day22, part2)]
fn part2(input: &utils::Brickfall) -> usize {
    (0..input.len())
        .map(|i| input.bricks_affected_by(i).filter(|&x| x).count())
        .sum()
}

mod utils {
    use itertools::Itertools;

    pub struct Brick {
        pub start: (usize, usize, usize),
        pub end: (usize, usize, usize),
    }

    impl std::str::FromStr for Brick {
        type Err = &'static str;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (start, end) = s.split_once('~').ok_or("Invalid brick")?;
            let start = start
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect_tuple()
                .ok_or("Invalid brick")?;
            let end = end
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect_tuple()
                .ok_or("Invalid brick")?;
            Ok(Brick { start, end })
        }
    }

    #[derive(derive_more::Deref)]
    pub struct Brickfall(Vec<Brick>);

    impl From<Vec<Brick>> for Brickfall {
        fn from(v: Vec<Brick>) -> Self {
            Self(v)
        }
    }

    impl Brickfall {
        pub fn apply_gravity(self) -> Self {
            let mut heightmap = rustc_hash::FxHashMap::default();
            self.0
                .into_iter()
                .map(|mut brick| {
                    // Find the highest point in the area below the brick
                    let max_z = (brick.start.0..=brick.end.0)
                        .cartesian_product(brick.start.1..=brick.end.1)
                        .map(|position| heightmap.get(&position).copied().unwrap_or_default())
                        .max()
                        .unwrap_or_default();

                    // Move the brick down to the highest point
                    let delta_z = brick.start.2 - max_z - 1;
                    brick.start.2 -= delta_z;
                    brick.end.2 -= delta_z;

                    // Update the heightmap with the height of the new brick
                    (brick.start.0..=brick.end.0)
                        .cartesian_product(brick.start.1..=brick.end.1)
                        .for_each(|position| {
                            heightmap.insert(position, brick.end.2);
                        });

                    brick
                })
                .collect_vec()
                .into()
        }

        /// Must be called after `apply_gravity`.
        pub fn bricks_affected_by(
            &self,
            analyzed_brick_index: usize,
        ) -> impl Iterator<Item = bool> + '_ {
            let mut heightmap = rustc_hash::FxHashMap::default();
            self.0
                .iter()
                .enumerate()
                // Skip the analyzed brick to see what other bricks are affected by it
                .filter_map(move |(i, brick)| (i != analyzed_brick_index).then_some(brick))
                .map(move |brick| {
                    // Find the highest point in the area below the brick
                    let max_z = (brick.start.0..=brick.end.0)
                        .cartesian_product(brick.start.1..=brick.end.1)
                        .map(|position| heightmap.get(&position).copied().unwrap_or_default())
                        .max()
                        .unwrap_or_default();

                    // Check if the brick is affected by the analyzed brick
                    let delta_z = brick.start.2 - max_z - 1;
                    let is_brick_affected = delta_z > 0;

                    // Update the heightmap with either the original or updated height
                    (brick.start.0..=brick.end.0)
                        .cartesian_product(brick.start.1..=brick.end.1)
                        .for_each(|position| {
                            heightmap.insert(
                                position,
                                if is_brick_affected {
                                    brick.end.2 - delta_z
                                } else {
                                    brick.end.2
                                },
                            );
                        });

                    is_brick_affected
                })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SAMPLE)), 5);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SAMPLE)), 7);
    }
}
