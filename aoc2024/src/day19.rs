use aoc_runner_derive::aoc;
use rayon::prelude::*;

fn parse(
    input: &str,
) -> (
    impl Iterator<Item = &str> + Clone,
    impl Iterator<Item = &str>,
) {
    let (available, required) = input.split_once("\n\n").unwrap();
    (available.split(", "), required.lines())
}

#[aoc(day19, part1)]
#[must_use]
pub fn part1(input: &str) -> usize {
    let (available, required) = parse(input);
    required
        .par_bridge()
        .filter(|towel| {
            get_n_permutations(towel, &available, &mut rustc_hash::FxHashMap::default()) > 0
        })
        .count()
}

#[aoc(day19, part2)]
#[must_use]
pub fn part2(input: &str) -> usize {
    let (available, required) = parse(input);
    required
        .par_bridge()
        .map(|towel| get_n_permutations(towel, &available, &mut rustc_hash::FxHashMap::default()))
        .sum()
}

fn get_n_permutations<'a>(
    required: &'a str,
    available: &(impl Iterator<Item = &'a str> + Clone),
    cache: &mut rustc_hash::FxHashMap<&'a str, usize>,
) -> usize {
    if let Some(&c) = cache.get(required) {
        return c;
    }
    if required.is_empty() {
        return 1;
    }
    let n_permutations = available
        .clone()
        .filter(|avail| avail.len() <= required.len() && required.starts_with(avail))
        .map(|avail| get_n_permutations(&required[avail.len()..], available, cache))
        .sum();
    cache.insert(required, n_permutations);
    n_permutations
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb
    "};

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(SAMPLE), 6);
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(part2(SAMPLE), 16);
    }
}
