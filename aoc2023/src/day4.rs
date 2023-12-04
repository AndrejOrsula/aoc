use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<utils::Card> {
    input
        .lines()
        .map(|line| {
            let mut numbers = line.split(':').last().unwrap().split('|');
            let winning_numbers = numbers
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|number| number.parse().unwrap())
                .collect();
            let our_numbers = numbers
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|number| number.parse().unwrap())
                .collect();
            utils::Card {
                winning_numbers,
                our_numbers,
            }
        })
        .collect()
}

mod utils {
    pub struct Card {
        pub winning_numbers: smallvec::SmallVec<[u8; 10]>,
        pub our_numbers: smallvec::SmallVec<[u8; 25]>,
    }

    impl Card {
        pub fn get_n_matches(&self) -> usize {
            self.our_numbers
                .iter()
                .filter(|n| self.winning_numbers.contains(n))
                .count()
        }
    }
}

#[aoc(day4, part1)]
fn part1(input: &[utils::Card]) -> u32 {
    input
        .iter()
        .map(|card| match card.get_n_matches() {
            0 => 0,
            n => 2_u32.pow(u32::try_from(n.saturating_sub(1)).unwrap()),
        })
        .sum()
}

#[aoc(day4, part2)]
fn part2(input: &[utils::Card]) -> u32 {
    let mut card_count: Vec<u32> = vec![1; input.len()];
    input
        .iter()
        .enumerate()
        .map(|(i, card)| {
            let current_card_count = card_count[i];
            card_count
                .iter_mut()
                .skip(i + 1)
                .take(card.get_n_matches())
                .for_each(|count| {
                    *count += current_card_count;
                });
            current_card_count
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SAMPLE)), 13);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SAMPLE)), 30);
    }
}
