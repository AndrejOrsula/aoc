use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse(input: &str) -> String {
    input.to_owned()
}

#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    // Sum of all numbers that are surrounded by symbols
    let mut sum = 0;

    // Iterate over the lines in the input
    for (current_line_index, current_line) in input.lines().enumerate() {
        let line_max_index = current_line.len().saturating_sub(1);

        // Try getting the lines above and below the current line
        let line_above = current_line_index
            .checked_sub(1)
            .and_then(|line_above_index| input.lines().nth(line_above_index));
        let line_below = input.lines().nth(current_line_index.saturating_add(1));

        // Iterate over the characters in the line and find the numbers
        let mut num_start = None;
        for (index, c) in current_line.char_indices() {
            // Mark the start a new number and continue until its end or the end of the line is reached
            let index_after_num = if c.is_ascii_digit() {
                if num_start.is_none() {
                    num_start = Some(index);
                }
                if index == line_max_index {
                    index.saturating_add(1)
                } else {
                    continue;
                }
            } else {
                index
            };

            // The full number is found once a non-digit character is encountered after start of the number was marked
            if let Some(number_index_start) = num_start {
                // Determine the index of the character before the number (for diagonal checks)
                let index_before_num = number_index_start.checked_sub(1);

                // Check if the number should be added to the sum
                #[allow(unused_parens)]
                if (
                    // Character to the left of the number
                    index_before_num.is_some_and(|index_before_num| {
                        utils::is_special_symbol(current_line.chars().nth(index_before_num).unwrap())
                    })
                    // Character to the right of the number
                    || current_line
                                .chars()
                                .nth(index_after_num).is_some_and(utils::is_special_symbol)
                    // Characters above the number
                    || line_above.is_some_and(|line_above| {
                        line_above[index_before_num.unwrap_or(0)..index_after_num.saturating_add(1).min(line_max_index)].chars()
                            .any(utils::is_special_symbol)
                    })
                    // Characters below the number
                    || line_below.is_some_and(|line_below| {
                        line_below[index_before_num.unwrap_or(0)..index_after_num.saturating_add(1).min(line_max_index)].chars()
                            .any(utils::is_special_symbol)
                    })
                ) {
                    // Add the number to the sum
                    sum += current_line[number_index_start..index_after_num]
                        .parse::<u32>()
                        .unwrap();
                }

                // Reset the start of the number
                num_start = None;
            }
        }
    }
    sum
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u32 {
    // Sum of all products between two numbers that are connected by a gear ('*')
    let mut sum = 0;

    // Find all numbers adjacent to the gear
    let mut adjacent_numbers = smallvec::SmallVec::<[u32; 2]>::new();

    // Iterate over the lines in the input
    for (current_line_index, current_line) in input.lines().enumerate() {
        // Skip lines that do not contain any gears
        if !current_line.contains(utils::is_gear) {
            continue;
        }

        // Try getting the lines above and below the current line
        let line_above = current_line_index
            .checked_sub(1)
            .and_then(|line_above_index| input.lines().nth(line_above_index));
        let line_below = input.lines().nth(current_line_index.saturating_add(1));

        // Iterate over the characters in the line and find the gears
        for (index, c) in current_line.char_indices() {
            // Skip non-gear characters
            if !utils::is_gear(c) {
                continue;
            }

            // Search for numbers to the left of the gear
            if let Some(number) = current_line[..index]
                .char_indices()
                .rev()
                .take_while(|(_, c)| c.is_ascii_digit())
                .last()
                .and_then(|(index, _)| {
                    current_line[index..]
                        .chars()
                        .take_while(char::is_ascii_digit)
                        .collect::<String>()
                        .parse::<u32>()
                        .ok()
                })
            {
                adjacent_numbers.push(number);
            }

            // Search for numbers to the right of the gear
            if let Ok(number) = current_line[(index + 1)..]
                .chars()
                .take_while(char::is_ascii_digit)
                .collect::<String>()
                .parse::<u32>()
            {
                adjacent_numbers.push(number);
            }

            // Search for numbers above the gear
            if let Some(line_above) = line_above {
                // Find the left-most digit above the gear
                let numbers_start_index = line_above[..=index.saturating_sub(1)]
                    .char_indices()
                    .rev()
                    .take_while(|(_, c)| c.is_ascii_digit())
                    .last()
                    .map_or(index.saturating_sub(1), |(i, _)| i);
                // Find the right-most digit above the gear
                let numbers_end_index = line_above[index.saturating_add(1)..]
                    .char_indices()
                    .take_while(|(_, c)| c.is_ascii_digit())
                    .last()
                    .map_or(index.saturating_add(1), |(i, _)| {
                        index.saturating_add(2) + i
                    });

                // Parse all numbers above the gear
                line_above[numbers_start_index..numbers_end_index]
                    .split(|c: char| !c.is_ascii_digit())
                    .filter_map(|number| number.parse::<u32>().ok())
                    .for_each(|number| adjacent_numbers.push(number));
            };

            // Search for numbers below the gear
            if let Some(line_below) = line_below {
                // Find the left-most digit below the gear
                let numbers_start_index = line_below[..=index.saturating_sub(1)]
                    .char_indices()
                    .rev()
                    .take_while(|(_, c)| c.is_ascii_digit())
                    .last()
                    .map_or(index.saturating_sub(1), |(i, _)| i);
                // Find the right-most digit below the gear
                let numbers_end_index = line_below[index.saturating_add(1)..]
                    .char_indices()
                    .take_while(|(_, c)| c.is_ascii_digit())
                    .last()
                    .map_or(index.saturating_add(1), |(i, _)| {
                        index.saturating_add(2) + i
                    });

                // Parse all numbers below the gear
                line_below[numbers_start_index..numbers_end_index]
                    .split(|c: char| !c.is_ascii_digit())
                    .filter_map(|number| number.parse::<u32>().ok())
                    .for_each(|number| adjacent_numbers.push(number));
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

mod utils {
    /// Returns `true` if the character is a special symbol (excluding '.').
    pub fn is_special_symbol(c: char) -> bool {
        c != '.' && c.is_ascii_graphic()
    }

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
        assert_eq!(part2(&parse(SAMPLE)), 467_835);
    }
}
