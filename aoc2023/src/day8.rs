use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day8)]
fn parse(input: &str) -> utils::Network {
    use itertools::Itertools;

    let mut lines = input.lines();
    let steps = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!("Invalid input"),
        })
        .collect();
    let mut map = std::collections::HashMap::new();
    for line in lines.skip(1) {
        let (key, steps) = line.split('=').collect_tuple().unwrap();
        let key = key.trim().to_string();
        let (left, right) = steps
            .split(',')
            .map(|s| {
                s.chars()
                    .filter(|c| c.is_alphanumeric())
                    .collect::<String>()
            })
            .collect_tuple()
            .unwrap();
        map.insert(key, [left, right]);
    }

    utils::Network { steps, map }
}

mod utils {
    pub struct Network {
        pub steps: Vec<usize>,
        pub map: std::collections::HashMap<String, [String; 2]>,
    }
}

#[aoc(day8, part1)]
fn part1(input: &utils::Network) -> u64 {
    // Begin at "AAA"
    let mut current_node = "AAA".to_string();

    // Keep looping until the end is reached
    let mut step_counter = 0;
    'outer: loop {
        // Iterate over all steps
        for step in &input.steps {
            step_counter += 1;

            // Update the current node
            current_node = input.map[&current_node][*step].to_string();

            // Break once the end is reached
            if current_node == "ZZZ" {
                break 'outer;
            }
        }
    }

    step_counter
}

#[aoc(day8, part2)]
fn part2(input: &utils::Network) -> u64 {
    use itertools::Itertools;
    use part2_utils::Path;

    // Find all nodes that end with 'A'
    let mut paths = input
        .map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(Path::new)
        .collect_vec();

    // Keep looping until all paths are finished
    let mut step_counter = 0;
    let mut paths_in_progress = (0..paths.len()).collect_vec();
    let mut finished_paths = smallvec::SmallVec::<[usize; 10]>::new();
    'outer: loop {
        // Iterate over all steps
        for step in &input.steps {
            step_counter += 1;

            // Update all paths in progress
            paths_in_progress.iter().enumerate().for_each(|(i, &path)| {
                let Path {
                    current_node,
                    steps_taken,
                } = &mut paths[path];
                *current_node = input.map[current_node][*step].to_string();
                if current_node.ends_with('Z') {
                    *steps_taken = step_counter;
                    finished_paths.push(i);
                }
            });

            // Remove finished paths from progress list
            for &i in &finished_paths {
                paths_in_progress.swap_remove(i);
            }

            // Break once all paths are finished
            if paths_in_progress.is_empty() {
                break 'outer;
            }
            finished_paths.clear();
        }
    }

    // Find the least common multiple of all paths
    paths[1..].iter().fold(
        paths[0].steps_taken,
        |acc,
         Path {
             current_node: _,
             steps_taken,
         }| { num::integer::lcm(acc, *steps_taken) },
    )
}

mod part2_utils {
    pub struct Path {
        pub current_node: String,
        pub steps_taken: u64,
    }

    impl Path {
        pub fn new(first_node: &impl ToString) -> Self {
            Self {
                current_node: first_node.to_string(),
                steps_taken: 0,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn part1_example() {
        const SAMPLE: &str = indoc! {"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        "};
        assert_eq!(part1(&parse(SAMPLE)), 6);
    }

    #[test]
    fn part2_example() {
        const SAMPLE: &str = indoc! {"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        "};
        assert_eq!(part2(&parse(SAMPLE)), 6);
    }
}
