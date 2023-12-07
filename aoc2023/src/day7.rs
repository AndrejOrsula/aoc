use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7, part1)]
fn parse1(input: &str) -> Vec<utils::CardHandWithBid<utils::part1::CamelCard>> {
    utils::parse(input)
}

#[aoc(day7, part1)]
fn part1(input: &[utils::CardHandWithBid<utils::part1::CamelCard>]) -> u64 {
    utils::solve(input)
}

#[aoc_generator(day7, part2)]
fn parse2(input: &str) -> Vec<utils::CardHandWithBid<utils::part2::CamelCard>> {
    utils::parse(input)
}

#[aoc(day7, part2)]
fn part2(input: &[utils::CardHandWithBid<utils::part2::CamelCard>]) -> u64 {
    utils::solve(input)
}

mod utils {
    use itertools::Itertools;
    use strum::{EnumIter, IntoEnumIterator};

    pub fn parse<T: Ord + Copy + TryFrom<char>>(input: &str) -> Vec<CardHandWithBid<T>>
    where
        HandType: From<[T; 5]>,
    {
        input
            .lines()
            .map(|line| {
                let (cards, bid) = line.split_ascii_whitespace().collect_tuple().unwrap();
                let cards: [T; 5] = cards
                    .chars()
                    .map(|card| {
                        card.try_into()
                            .unwrap_or_else(|_| panic!("Invalid card: {card}"))
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap_or_else(|_| panic!("Invalid hand: {cards}"));
                let hand_type = cards.into();
                CardHandWithBid {
                    hand: CardHand { hand_type, cards },
                    bid: bid.parse().unwrap(),
                }
            })
            .collect()
    }

    pub fn solve<T: Ord + Copy>(input: &[CardHandWithBid<T>]) -> u64 {
        input
            .iter()
            .sorted_by_key(|card_hand| card_hand.hand)
            .enumerate()
            .map(|(i, card_hand)| u64::try_from(i + 1).unwrap() * card_hand.bid)
            .sum()
    }

    pub struct CardHandWithBid<T: Ord> {
        pub hand: CardHand<T>,
        pub bid: u64,
    }

    #[derive(Clone, Copy, Eq)]
    pub struct CardHand<T: Ord> {
        pub hand_type: HandType,
        pub cards: [T; 5],
    }

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum HandType {
        HighCard,
        OnePair,
        TwoPair,
        ThreeOfKind,
        FullHouse,
        FourOfKind,
        FiveOfKind,
    }

    impl<T: Ord> PartialEq for CardHand<T> {
        fn eq(&self, other: &Self) -> bool {
            self.cards.eq(&other.cards)
        }
    }

    impl<T: Ord> PartialOrd for CardHand<T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl<T: Ord> Ord for CardHand<T> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            match self.hand_type.cmp(&other.hand_type) {
                std::cmp::Ordering::Equal => {
                    for i in 0..5 {
                        match self.cards[i].cmp(&other.cards[i]) {
                            std::cmp::Ordering::Equal => {}
                            other => return other,
                        }
                    }
                    std::cmp::Ordering::Equal
                }
                other => other,
            }
        }
    }

    pub mod part1 {
        use super::{EnumIter, HandType, IntoEnumIterator};

        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
        pub enum CamelCard {
            Two,
            Three,
            Four,
            Five,
            Six,
            Seven,
            Eight,
            Nine,
            T,
            J,
            Q,
            K,
            A,
        }

        impl TryFrom<char> for CamelCard {
            type Error = &'static str;
            fn try_from(value: char) -> Result<Self, Self::Error> {
                match value {
                    '2' => Ok(Self::Two),
                    '3' => Ok(Self::Three),
                    '4' => Ok(Self::Four),
                    '5' => Ok(Self::Five),
                    '6' => Ok(Self::Six),
                    '7' => Ok(Self::Seven),
                    '8' => Ok(Self::Eight),
                    '9' => Ok(Self::Nine),
                    'T' => Ok(Self::T),
                    'J' => Ok(Self::J),
                    'Q' => Ok(Self::Q),
                    'K' => Ok(Self::K),
                    'A' => Ok(Self::A),
                    _ => Err("Invalid card"),
                }
            }
        }

        impl From<[CamelCard; 5]> for HandType {
            fn from(value: [CamelCard; 5]) -> Self {
                let mut hand_types_raw = arrayvec::ArrayVec::<Self, 2>::new();
                for card in CamelCard::iter() {
                    match value.iter().filter(|&&c| c == card).count() {
                        2 => hand_types_raw.push(Self::OnePair),
                        3 => hand_types_raw.push(Self::ThreeOfKind),
                        4 => {
                            hand_types_raw.push(Self::FourOfKind);
                            break;
                        }
                        5 => {
                            hand_types_raw.push(Self::FiveOfKind);
                            break;
                        }
                        _ => {}
                    }
                }
                match hand_types_raw.len() {
                    0 => Self::HighCard,
                    1 => hand_types_raw[0],
                    2 => {
                        hand_types_raw.sort();
                        match hand_types_raw[0] {
                            Self::OnePair => match hand_types_raw[1] {
                                Self::OnePair => Self::TwoPair,
                                Self::ThreeOfKind => Self::FullHouse,
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    pub mod part2 {
        use super::{EnumIter, HandType, IntoEnumIterator};

        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
        pub enum CamelCard {
            J,
            Two,
            Three,
            Four,
            Five,
            Six,
            Seven,
            Eight,
            Nine,
            T,
            Q,
            K,
            A,
        }

        impl TryFrom<char> for CamelCard {
            type Error = &'static str;
            fn try_from(value: char) -> Result<Self, Self::Error> {
                match value {
                    'J' => Ok(Self::J),
                    '2' => Ok(Self::Two),
                    '3' => Ok(Self::Three),
                    '4' => Ok(Self::Four),
                    '5' => Ok(Self::Five),
                    '6' => Ok(Self::Six),
                    '7' => Ok(Self::Seven),
                    '8' => Ok(Self::Eight),
                    '9' => Ok(Self::Nine),
                    'T' => Ok(Self::T),
                    'Q' => Ok(Self::Q),
                    'K' => Ok(Self::K),
                    'A' => Ok(Self::A),
                    _ => Err("Invalid card"),
                }
            }
        }

        impl From<[CamelCard; 5]> for HandType {
            fn from(value: [CamelCard; 5]) -> Self {
                let mut hand_types_raw = arrayvec::ArrayVec::<Self, 2>::new();
                let mut n_jokers = 0;
                for card in CamelCard::iter() {
                    let n_cards = value.iter().filter(|&&c| c == card).count();
                    if card == CamelCard::J {
                        n_jokers = n_cards;
                    }
                    match n_cards {
                        2 => hand_types_raw.push(Self::OnePair),
                        3 => hand_types_raw.push(Self::ThreeOfKind),
                        4 => {
                            hand_types_raw.push(Self::FourOfKind);
                            break;
                        }
                        5 => {
                            hand_types_raw.push(Self::FiveOfKind);
                            break;
                        }
                        _ => {}
                    }
                }
                let hand_type = match hand_types_raw.len() {
                    0 => Self::HighCard,
                    1 => hand_types_raw[0],
                    2 => {
                        hand_types_raw.sort();
                        match hand_types_raw[0] {
                            Self::OnePair => match hand_types_raw[1] {
                                Self::OnePair => Self::TwoPair,
                                Self::ThreeOfKind => Self::FullHouse,
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        }
                    }
                    _ => unreachable!(),
                };
                match (hand_type, n_jokers) {
                    (Self::FourOfKind, 1 | 4) | (Self::FullHouse, 2 | 3) => Self::FiveOfKind,
                    (Self::FullHouse, 1) | (Self::ThreeOfKind, 1 | 3) | (Self::TwoPair, 2) => {
                        Self::FourOfKind
                    }
                    (Self::TwoPair, 1) => Self::FullHouse,
                    (Self::OnePair, 1 | 2) => Self::ThreeOfKind,
                    (Self::HighCard, 1) => Self::OnePair,
                    (hand, _) => hand,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse1(SAMPLE)), 6440);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse2(SAMPLE)), 5905);
    }
}
