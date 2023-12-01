use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse(input: &str) -> String {
    input.to_owned()
}

#[aoc(day1, part1)]
fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter_map(|c| c.to_digit(10));
            let first_digit = digits.next().unwrap();
            let last_digit = digits.last().unwrap_or(first_digit);

            first_digit * 10 + last_digit
        })
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &str) -> u32 {
    use part2_utils::*;

    input
        .lines()
        .map(|line| {
            let digits = Digit::find_all_firsts_and_lasts(line);
            let first_digit: u32 = digits.get(digits.keys().min().unwrap()).unwrap().into();
            let last_digit: u32 = digits.get(digits.keys().max().unwrap()).unwrap().into();
            first_digit * 10 + last_digit
        })
        .sum()
}

mod part2_utils {
    use std::collections::HashMap;
    use strum::IntoEnumIterator;

    #[repr(u32)]
    #[derive(Debug, Clone, Copy, strum::EnumIter)]
    /// Represents a digit from 1 to 9.
    pub enum Digit {
        One = 1,
        Two = 2,
        Three = 3,
        Four = 4,
        Five = 5,
        Six = 6,
        Seven = 7,
        Eight = 8,
        Nine = 9,
    }

    impl Digit {
        /// Finds the first and last instances of all `Digit`s in a string.
        /// Digit can be either spelled out or in numeric form.
        ///
        /// # Returns
        /// HashMap in the form of <index_of_the_first_letter: usize, digit: Digit>.
        pub fn find_all_firsts_and_lasts(input: &str) -> HashMap<usize, Self> {
            let mut result = HashMap::new();

            for digit in Self::iter() {
                let digit_spelling = digit.as_spelling();
                let digit_char = digit.as_char();

                // Get index to the first instance of the digit
                let first_index = {
                    let first_index_spelling = input.find(digit_spelling);
                    let first_index_char = input.find(digit_char);

                    match (first_index_spelling, first_index_char) {
                        (Some(spelling), Some(char)) => spelling.min(char),
                        (Some(spelling), None) => spelling,
                        (None, Some(char)) => char,
                        (None, None) => continue,
                    }
                };
                // Insert the digit into the result
                result.insert(first_index, digit);

                // Get index to the last instance of the digit
                let last_index = {
                    let last_index_spelling = input.rfind(digit_spelling);
                    let last_index_char = input.rfind(digit_char);

                    match (last_index_spelling, last_index_char) {
                        (Some(spelling), Some(char)) => spelling.max(char),
                        (Some(spelling), None) => spelling,
                        (None, Some(char)) => char,
                        _ => unreachable!(),
                    }
                };
                // Insert the digit into the result if the last index is different from the first
                if last_index != first_index {
                    result.insert(last_index, digit);
                }
            }

            result
        }

        /// Returns the spelling of the digit.
        fn as_spelling(&self) -> &'static str {
            match self {
                Digit::One => "one",
                Digit::Two => "two",
                Digit::Three => "three",
                Digit::Four => "four",
                Digit::Five => "five",
                Digit::Six => "six",
                Digit::Seven => "seven",
                Digit::Eight => "eight",
                Digit::Nine => "nine",
            }
        }

        /// Returns the digit as a char.
        fn as_char(&self) -> char {
            match self {
                Digit::One => '1',
                Digit::Two => '2',
                Digit::Three => '3',
                Digit::Four => '4',
                Digit::Five => '5',
                Digit::Six => '6',
                Digit::Seven => '7',
                Digit::Eight => '8',
                Digit::Nine => '9',
            }
        }
    }

    impl From<Digit> for u32 {
        fn from(value: Digit) -> Self {
            value as u32
        }
    }

    impl From<&Digit> for u32 {
        fn from(value: &Digit) -> Self {
            (*value).into()
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
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "};
        assert_eq!(part1(&parse(SAMPLE)), 142);
    }

    #[test]
    fn part2_example() {
        const SAMPLE: &str = indoc! {"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "};
        assert_eq!(part2(&parse(SAMPLE)), 281);
    }
}
