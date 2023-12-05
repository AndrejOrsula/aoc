use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn parse(input: &str) -> utils::Almanac {
    use itertools::Itertools;

    let mut input_caterogies = input.split("\n\n");
    let seeds = input_caterogies
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mappings = input_caterogies
        .map(|category| {
            category
                .lines()
                .skip(1)
                .map(|line| {
                    let (destination_start, source_start, range_length) = line
                        .split_ascii_whitespace()
                        .map(|s| s.parse().unwrap())
                        .next_tuple()
                        .unwrap();
                    utils::RangeMap {
                        from: std::ops::Range {
                            start: source_start,
                            end: source_start.checked_add(range_length).unwrap(),
                        },
                        to: std::ops::Range {
                            start: destination_start,
                            end: destination_start.checked_add(range_length).unwrap(),
                        },
                    }
                })
                .collect()
        })
        .collect();
    utils::Almanac { seeds, mappings }
}

mod utils {
    pub struct Almanac {
        pub seeds: smallvec::SmallVec<[u64; 20]>,
        pub mappings: smallvec::SmallVec<[Vec<RangeMap>; 7]>,
    }

    pub struct RangeMap {
        pub from: std::ops::Range<u64>,
        pub to: std::ops::Range<u64>,
    }
}

#[aoc(day5, part1)]
fn part1(input: &utils::Almanac) -> u64 {
    let utils::Almanac { seeds, mappings } = input;
    seeds
        .iter()
        .map(|&seed| {
            mappings.iter().fold(seed, |value, mapping| {
                mapping
                    .iter()
                    .find(|&range_map| range_map.from.contains(&value))
                    .map_or(value, |range_map| {
                        value + range_map.to.start - range_map.from.start
                    })
            })
        })
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &utils::Almanac) -> u64 {
    use rayon::prelude::*;

    let utils::Almanac { seeds, mappings } = input;
    seeds
        .chunks(2)
        .map(|chunk| {
            let (seed_range_start, seed_range_end) = (chunk[0], chunk[0] + chunk[1]);
            seed_range_start..seed_range_end
        })
        .map(|seed_range| {
            seed_range
                .into_par_iter()
                .map(|seed| {
                    mappings.iter().fold(seed, |value, mapping| {
                        mapping
                            .iter()
                            .find(|&range_map| range_map.from.contains(&value))
                            .map_or(value, |range_map| {
                                value + range_map.to.start - range_map.from.start
                            })
                    })
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SAMPLE)), 35);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SAMPLE)), 46);
    }
}
