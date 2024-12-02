use aoc_runner_derive::aoc;

#[inline]
fn parse_line(line: &str) -> (u32, u32) {
    let (l, r) = line.split_once(|c: char| c.is_ascii_whitespace()).unwrap();
    let l = l.parse().unwrap();
    let r = r.trim_ascii_start().parse().unwrap();
    (l, r)
}

#[aoc(day1, part1)]
#[must_use]
pub fn part1(input: &str) -> u32 {
    let mut left = smallvec::SmallVec::<[u32; 1024]>::new();
    let mut right = smallvec::SmallVec::<[u32; 1024]>::new();
    input.lines().for_each(|line| {
        let (l, r) = parse_line(line);

        left.insert(left.binary_search(&l).unwrap_or_else(|e| e), l);
        right.insert(right.binary_search(&r).unwrap_or_else(|e| e), r);
    });

    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

#[aoc(day1, part2)]
#[must_use]
pub fn part2(input: &str) -> u32 {
    let mut left = smallvec::SmallVec::<[u32; 1024]>::new();
    let mut right_count =
        rustc_hash::FxHashMap::with_capacity_and_hasher(896, rustc_hash::FxBuildHasher);
    input.lines().for_each(|line| {
        let (l, r) = parse_line(line);

        left.insert(left.binary_search(&l).unwrap_or_else(|e| e), l);
        *right_count.entry(r).or_insert(0) += 1;
    });

    left.into_iter()
        .filter_map(|l| right_count.get(&l).map(|count| l * count))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    "};

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(SAMPLE), 11);
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(part2(SAMPLE), 31);
    }
}
