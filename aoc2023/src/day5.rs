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
                        to_start: destination_start,
                    }
                })
                .collect()
        })
        .collect();
    utils::Almanac { seeds, mappings }
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
                        value + range_map.to_start - range_map.from.start
                    })
            })
        })
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &utils::Almanac) -> u64 {
    let utils::Almanac { seeds, mappings } = input;
    seeds
        .chunks(2)
        .map(|nums| nums[0]..(nums[0] + nums[1]))
        .flat_map(|seed_range| {
            mappings.iter().fold(
                smallvec::SmallVec::<[_; 32]>::from_elem(seed_range, 1),
                |seed_ranges, mappings| {
                    let mut mapped_ranges = smallvec::SmallVec::new();
                    let leftover_ranges =
                        mappings.iter().fold(seed_ranges, |seed_ranges, mapping| {
                            seed_ranges
                                .into_iter()
                                .flat_map(|seed_range| {
                                    if seed_range.start >= mapping.from.end
                                        || seed_range.end <= mapping.from.start
                                    {
                                        [seed_range, std::ops::Range::default()]
                                    } else if seed_range.start <= mapping.from.start
                                        && seed_range.end >= mapping.from.end
                                    {
                                        let mapped = mapping.to_start
                                            ..mapping.to_start + mapping.from.end
                                                - mapping.from.start;
                                        if mapped.start < mapped.end {
                                            mapped_ranges.push(mapped);
                                        }

                                        [
                                            seed_range.start..mapping.from.start,
                                            mapping.from.end..seed_range.end,
                                        ]
                                    } else if seed_range.start >= mapping.from.start
                                        && seed_range.end <= mapping.from.end
                                    {
                                        let mapped = seed_range.start - mapping.from.start
                                            + mapping.to_start
                                            ..seed_range.end - mapping.from.start
                                                + mapping.to_start;
                                        if mapped.start < mapped.end {
                                            mapped_ranges.push(mapped);
                                        }

                                        [std::ops::Range::default(), std::ops::Range::default()]
                                    } else {
                                        let mapped = seed_range.start.max(mapping.from.start)
                                            - mapping.from.start
                                            + mapping.to_start
                                            ..seed_range.end.min(mapping.from.end)
                                                - mapping.from.start
                                                + mapping.to_start;
                                        if mapped.start < mapped.end {
                                            mapped_ranges.push(mapped);
                                        }

                                        let leftover_start =
                                            if seed_range.start < mapping.from.start {
                                                seed_range.start
                                            } else {
                                                mapping.from.end
                                            };
                                        let leftover_end = if seed_range.end > mapping.from.end {
                                            seed_range.end
                                        } else {
                                            mapping.from.start
                                        };
                                        [leftover_start..leftover_end, std::ops::Range::default()]
                                    }
                                })
                                .filter(|range| range.start < range.end)
                                .collect()
                        });
                    mapped_ranges.extend(leftover_ranges);
                    mapped_ranges
                },
            )
        })
        .map(|range| range.start)
        .min()
        .unwrap()
}

mod utils {
    pub struct Almanac {
        pub seeds: smallvec::SmallVec<[u64; 20]>,
        pub mappings: smallvec::SmallVec<[Vec<RangeMap>; 7]>,
    }

    pub struct RangeMap {
        pub from: std::ops::Range<u64>,
        pub to_start: u64,
    }
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
