use aoc_runner_derive::aoc;

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
    let mut map = rustc_hash::FxHashMap::default();
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

#[aoc(day8, part1)]
#[must_use]
pub fn part1(input: &str) -> u64 {
    let input = parse(input);
    // Begin at "AAA"
    let mut current_node = "AAA";

    // Keep looping until the end is reached
    let mut step_counter = 0;
    loop {
        // Iterate over all steps
        for &step in &input.steps {
            step_counter += 1;

            // Update the current node
            current_node = input.map[current_node][step].as_str();

            // Return once the end is reached
            if current_node == "ZZZ" {
                return step_counter;
            }
        }
    }
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> u64 {
    let input = parse(input);
    // Find all nodes that end with 'A'
    let mut nodes: smallvec::SmallVec<[&str; 8]> = input
        .map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(String::as_str)
        .collect();

    // Keep track of the lowest common multiple of all paths
    let mut lcs = 1;

    // Keep looping until all paths are finished
    let mut step_counter = 0;
    let mut finished_paths = smallvec::SmallVec::<[usize; 2]>::new();
    loop {
        // Iterate over all steps
        for &step in &input.steps {
            step_counter += 1;

            // Update all active paths
            nodes.iter_mut().enumerate().for_each(|(i, node)| {
                *node = input.map[*node][step].as_str();
                if node.ends_with('Z') {
                    finished_paths.push(i);
                }
            });

            if !finished_paths.is_empty() {
                // Update the lowest common multiple
                lcs = num::integer::lcm(lcs, step_counter);

                // Return once all nodes are finished
                if nodes.len() == finished_paths.len() {
                    return lcs;
                }

                // Remove all finished nodes
                while let Some(i) = finished_paths.pop() {
                    nodes.swap_remove(i);
                }
            }
        }
    }
}

mod utils {
    pub struct Network {
        pub steps: Vec<usize>,
        pub map: rustc_hash::FxHashMap<String, [String; 2]>,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    pub fn part1_example() {
        const SAMPLE: &str = indoc! {"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        "};
        assert_eq!(part1(SAMPLE), 6);
    }

    #[test]
    pub fn part2_example() {
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
        assert_eq!(part2(SAMPLE), 6);
    }
}
