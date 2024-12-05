use aoc_runner_derive::aoc;

fn parse_input(
    input: &str,
) -> (
    rustc_hash::FxHashSet<(u8, u8)>,
    impl Iterator<Item = smallvec::SmallVec<[u8; 23]>> + '_,
) {
    let rules: rustc_hash::FxHashSet<(u8, u8)> = input
        .lines()
        .take_while(|line| !line.is_empty())
        .filter_map(|line| {
            line.split_once('|')
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        })
        .collect();
    let updates = input.lines().skip(rules.len() + 1).map(|update| {
        update
            .split(',')
            .filter_map(|num| num.parse().ok())
            .collect()
    });
    (rules, updates)
}

#[aoc(day5, part1)]
#[must_use]
pub fn part1(input: &str) -> u16 {
    let (rules, updates) = parse_input(input);
    updates
        .filter_map(|update| {
            update
                .is_sorted_by(|a, b| !rules.contains(&(*b, *a)))
                .then_some(u16::from(update[update.len() / 2]))
        })
        .sum()
}

#[aoc(day5, part2)]
#[must_use]
pub fn part2(input: &str) -> u16 {
    let (rules, updates) = parse_input(input);
    updates
        .filter_map(|mut update| {
            (!update.is_sorted_by(|a, b| !rules.contains(&(*b, *a)))).then_some({
                update.sort_by(|a, b| {
                    if rules.contains(&(*a, *b)) {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Greater
                    }
                });
                u16::from(update[update.len() / 2])
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    "};

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(SAMPLE), 143);
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(part2(SAMPLE), 123);
    }
}
