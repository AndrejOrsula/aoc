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
            let last_digit = digits.next_back().unwrap_or(first_digit);

            10 * first_digit + last_digit
        })
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &str) -> u32 {
    use part2_utils::Digit;

    input
        .lines()
        .map(|line| {
            let (first_digit, last_digit) = Digit::find_first_and_last(line);

            10 * (first_digit as u32) + (last_digit as u32)
        })
        .sum()
}

mod part2_utils {
    use strum::{EnumIter, IntoEnumIterator};

    #[repr(u32)]
    #[derive(Debug, Clone, Copy, EnumIter)]
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
        /// Finds the first and last instances of `Digit` in a string.
        /// Digit can be either spelled out or in numeric form.
        ///
        /// # Returns
        /// (Digit, Digit) in the form of (first digit, last digit).
        pub fn find_first_and_last(input: &str) -> (Self, Self) {
            let mut first_index = usize::MAX;
            let mut last_index = usize::MIN;

            let mut first_digit = Self::One;
            let mut last_digit = Self::One;

            for digit in Self::iter() {
                let digit_spelling = digit.as_spelling();
                let digit_char = digit.as_char();

                // Get index to the first instance of the digit
                let digit_first_index = {
                    match (input.find(digit_spelling), input.find(digit_char)) {
                        (Some(s), Some(c)) => s.min(c),
                        (Some(s), None) => s,
                        (None, Some(c)) => c,
                        (None, None) => continue,
                    }
                };
                // Get index to the last instance of the digit
                let digit_last_index = {
                    match (input.rfind(digit_spelling), input.rfind(digit_char)) {
                        (Some(s), Some(c)) => s.max(c),
                        (Some(s), None) => s,
                        (None, Some(c)) => c,
                        _ => unreachable!(),
                    }
                };

                // Update the first and last indices
                if digit_first_index <= first_index {
                    first_index = digit_first_index;
                    first_digit = digit;
                }
                if digit_last_index >= last_index {
                    last_index = digit_last_index;
                    last_digit = digit;
                }
            }

            (first_digit, last_digit)
        }

        /// Returns the spelling of the digit.
        fn as_spelling(self) -> &'static str {
            match self {
                Self::One => "one",
                Self::Two => "two",
                Self::Three => "three",
                Self::Four => "four",
                Self::Five => "five",
                Self::Six => "six",
                Self::Seven => "seven",
                Self::Eight => "eight",
                Self::Nine => "nine",
            }
        }

        /// Returns the digit as a char.
        fn as_char(self) -> char {
            match self {
                Self::One => '1',
                Self::Two => '2',
                Self::Three => '3',
                Self::Four => '4',
                Self::Five => '5',
                Self::Six => '6',
                Self::Seven => '7',
                Self::Eight => '8',
                Self::Nine => '9',
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
