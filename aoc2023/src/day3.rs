use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse(input: &str) -> String {
    input.to_owned()
}

#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    use part1_utils::is_special_symbol;

    // Sum of all numbers that are surrounded by symbols
    let mut sum = 0;

    // Iterate over the lines in the input
    for (current_line_index, current_line_content) in input.lines().enumerate() {
        // Determine the index of the lines before and after the number
        let line_above_index = current_line_index.checked_sub(1);
        let line_below_index = current_line_index + 1;

        // Iterate over the characters in the line and find the numbers
        let mut number_start = None;
        let mut char_indices_peekable = current_line_content.char_indices().peekable();
        while let Some((mut index, c)) = char_indices_peekable.next() {
            // Mark the start a new number and continue until its end or the end of the line is reached
            if c.is_ascii_digit() {
                if number_start.is_none() {
                    number_start = Some(index);
                }
                if char_indices_peekable.peek().is_some() {
                    continue;
                } else {
                    index += 1;
                }
            }

            // The full number is found once a non-digit character is encountered after start of the number was marked
            if let Some(number_index_start) = number_start {
                // Determine the index of the character before and after the number
                let char_index_before = number_index_start.checked_sub(1);
                let char_index_after = index;

                // Check if the number should be added to the sum
                #[allow(unused_parens)]
                if (
                    // Character to the left of the number
                    char_index_before.is_some_and(|char_index_before| {
                        current_line_content
                                .chars()
                                .nth(char_index_before).is_some_and(is_special_symbol)
                    })
                    // Character to the right of the number
                    || current_line_content
                                .chars()
                                .nth(char_index_after).is_some_and(is_special_symbol)
                    // Characters above the number
                    || line_above_index.is_some_and(|line_above_index| {
                        input
                            .lines()
                            .nth(line_above_index)
                            .map_or(false, |line| {
                                line.chars().skip(char_index_before.unwrap_or(0))
                                    .take((char_index_after - char_index_before.unwrap_or(0))
                                    .saturating_add(1))
                                    .any(is_special_symbol)
                            })
                    })
                    // Characters below the number
                    || input
                        .lines()
                        .nth(line_below_index)
                        .map_or(false, |line| {
                            line.chars()
                                .skip(char_index_before.unwrap_or(0))
                                .take((char_index_after - char_index_before.unwrap_or(0)).saturating_add(1))
                                .any(is_special_symbol)
                        })
                ) {
                    // Add the number to the sum
                    sum += current_line_content[number_index_start..char_index_after]
                        .parse::<u32>()
                        .unwrap();
                }

                // Reset the start of the number
                number_start = None;
            }
        }
    }
    sum
}

mod part1_utils {
    /// Returns `true` if the character is a special symbol (excluding '.').
    pub fn is_special_symbol(c: char) -> bool {
        c != '.' && c.is_ascii_graphic()
    }
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u32 {
    // Sum of all products between two numbers that are connected by a gear ('*')
    let mut sum = 0;

    // Find all numbers adjacent to the gear
    let mut adjacent_numbers = Vec::with_capacity(2);

    // Iterate over the lines in the input
    for (current_line_index, current_line_content) in input.lines().enumerate() {
        // Determine the index of the lines before and after the gear
        let line_above_index = current_line_index.checked_sub(1);
        let line_below_index = current_line_index + 1;

        // Iterate over the characters in the line and find the gears
        let char_indices_peekable = current_line_content.char_indices().peekable();
        for (index, c) in char_indices_peekable {
            // Skip non-gear characters
            if !part2_utils::is_gear(c) {
                continue;
            }

            // Search for numbers to the left of the gear
            if let Some(number) = current_line_content[..index]
                .char_indices()
                .rev()
                .take_while(|(_, c)| c.is_ascii_digit())
                .last()
                .and_then(|(index, _)| {
                    current_line_content[index..]
                        .chars()
                        .take_while(|c| c.is_ascii_digit())
                        .collect::<String>()
                        .parse::<u32>()
                        .ok()
                })
            {
                adjacent_numbers.push(number)
            }

            // Search for numbers to the right of the gear
            if let Ok(number) = current_line_content[(index + 1)..]
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<u32>()
            {
                adjacent_numbers.push(number)
            }

            // Search for numbers above the gear
            if let Some(line_above_index) = line_above_index {
                // Find the left-most digit above the gear
                let numbers_start_index = input
                    .lines()
                    .nth(line_above_index)
                    .and_then(|line| {
                        line[..=index.saturating_sub(1)]
                            .char_indices()
                            .rev()
                            .take_while(|(_, c)| c.is_ascii_digit())
                            .last()
                            .map(|(i, _)| i)
                    })
                    .unwrap_or(index.saturating_sub(1));
                // Find the right-most digit above the gear
                let numbers_end_index = input
                    .lines()
                    .nth(line_above_index)
                    .and_then(|line| {
                        line[index.saturating_add(1)..]
                            .char_indices()
                            .take_while(|(_, c)| c.is_ascii_digit())
                            .last()
                            .map(|(i, _)| index.saturating_add(2) + i)
                    })
                    .unwrap_or(index.saturating_add(1));

                // Parse all numbers above the gear
                if let Some(line) = input.lines().nth(line_above_index) {
                    line[numbers_start_index..numbers_end_index]
                        .split(|c: char| !c.is_ascii_digit())
                        .filter_map(|number| number.parse::<u32>().ok())
                        .for_each(|number| adjacent_numbers.push(number))
                }
            };

            // Search for numbers below the gear
            {
                // Find the left-most digit below the gear
                let numbers_start_index = input
                    .lines()
                    .nth(line_below_index)
                    .and_then(|line| {
                        line[..=index.saturating_sub(1)]
                            .char_indices()
                            .rev()
                            .take_while(|(_, c)| c.is_ascii_digit())
                            .last()
                            .map(|(i, _)| i)
                    })
                    .unwrap_or(index.saturating_sub(1));
                // Find the right-most digit below the gear
                let numbers_end_index = input
                    .lines()
                    .nth(line_below_index)
                    .and_then(|line| {
                        line[index.saturating_add(1)..]
                            .char_indices()
                            .take_while(|(_, c)| c.is_ascii_digit())
                            .last()
                            .map(|(i, _)| index.saturating_add(2) + i)
                    })
                    .unwrap_or(index.saturating_add(1));

                // Parse all numbers below the gear
                if let Some(line) = input.lines().nth(line_below_index) {
                    line[numbers_start_index..numbers_end_index]
                        .split(|c: char| !c.is_ascii_digit())
                        .filter_map(|number| number.parse::<u32>().ok())
                        .for_each(|number| adjacent_numbers.push(number))
                }
            }

            // Add the product of the adjacent numbers to the sum if there are exactly two adjacent numbers
            if adjacent_numbers.len() == 2 {
                sum += adjacent_numbers[0] * adjacent_numbers[1];
            }
            adjacent_numbers.clear();
        }
    }
    sum
}

mod part2_utils {
    /// Returns `true` if the character is a gear ('*').
    pub fn is_gear(c: char) -> bool {
        c == '*'
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SAMPLE)), 4361);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SAMPLE)), 467835);
    }
}
