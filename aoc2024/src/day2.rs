use aoc_runner_derive::aoc;

#[inline]
fn parse_line(line: &str) -> smallvec::SmallVec<[u8; 8]> {
    line.split_ascii_whitespace()
        .map(|num| num.parse().unwrap())
        .collect()
}

#[inline]
fn check_values(values: &[u8]) -> bool {
    let mut inc = true;
    let mut dec = true;
    for win in values.windows(2) {
        if win[0].abs_diff(win[1]) > 3 {
            return false;
        }
        inc &= win[0] < win[1];
        dec &= win[0] > win[1];
        if !(inc ^ dec) {
            return false;
        }
    }
    true
}

#[aoc(day2, part1)]
#[must_use]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| check_values(&parse_line(line)))
        .count()
}

#[aoc(day2, part2)]
#[must_use]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let values = parse_line(line);
            check_values(&values)
                || (0..values.len()).any(|i| {
                    let mut values = values.clone();
                    values.remove(i);
                    check_values(&values)
                })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    "};

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(SAMPLE), 2);
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(part2(SAMPLE), 4);
    }
}
