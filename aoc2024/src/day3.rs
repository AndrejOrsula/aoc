use aoc_runner_derive::aoc;

#[aoc(day3, part1)]
#[must_use]
pub fn part1(input: &str) -> u32 {
    let mut result = 0;

    let mut word = arrayvec::ArrayString::<11>::new();
    input.lines().for_each(|line| {
        line.chars().for_each(|c| match c {
            'm' => {
                word.push('m');
            }
            'u' if &word == "m" => {
                word.push('u');
            }
            'l' if &word == "mu" => {
                word.push('l');
            }
            '(' if &word == "mul" => {
                word.push('(');
            }
            d if d.is_ascii_digit()
                && word.starts_with("mul(")
                && word.chars().skip(4).all(|c| c.is_ascii_digit() || c == ',') =>
            {
                word.push(d);
            }
            ',' if word.starts_with("mul(") && word.chars().skip(4).all(|c| c.is_ascii_digit()) => {
                word.push(',');
            }
            ')' if word.starts_with("mul(") => {
                if let Some((a, b)) = word[4..].split_once(',') {
                    if let (Ok(a), Ok(b)) = (a.parse::<u32>(), b.parse::<u32>()) {
                        result += a * b;
                    }
                }
                word.clear();
            }
            _ => {
                word.clear();
            }
        });
    });

    result
}

#[aoc(day3, part2)]
#[must_use]
pub fn part2(input: &str) -> u32 {
    let mut result = 0;

    let mut word = arrayvec::ArrayString::<11>::new();
    let mut enabled = true;
    input.lines().for_each(|line| {
        line.chars().for_each(|c| match c {
            'm' if enabled => {
                word.push('m');
            }
            'u' if enabled && &word == "m" => {
                word.push('u');
            }
            'l' if enabled && &word == "mu" => {
                word.push('l');
            }
            'd' => {
                word.push('d');
            }
            'o' if &word == "d" => {
                word.push('o');
            }
            'n' if enabled && &word == "do" => {
                word.push('n');
            }
            '\'' if enabled && &word == "don" => {
                word.push('\'');
            }
            't' if enabled && &word == "don'" => {
                word.push('t');
            }
            '(' if (enabled && (&word == "mul" || &word == "don't")) || &word == "do" => {
                word.push('(');
            }
            d if d.is_ascii_digit()
                && enabled
                && word.starts_with("mul(")
                && word.chars().skip(4).all(|c| c.is_ascii_digit() || c == ',') =>
            {
                word.push(d);
            }
            ',' if enabled
                && word.starts_with("mul(")
                && word.chars().skip(4).all(|c| c.is_ascii_digit()) =>
            {
                word.push(',');
            }
            ')' => {
                if enabled {
                    if let Some(word) = word.strip_prefix("mul(") {
                        if let Some((a, b)) = word.split_once(',') {
                            if let (Ok(a), Ok(b)) = (a.parse::<u32>(), b.parse::<u32>()) {
                                result += a * b;
                            }
                        }
                    } else if &word == "don't(" {
                        enabled = false;
                    }
                } else if &word == "do(" {
                    enabled = true;
                }
                word.clear();
            }
            _ => {
                word.clear();
            }
        });
    });

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    pub fn part1_example() {
        const SAMPLE: &str = indoc! {"
            mul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
        "};
        assert_eq!(part1(SAMPLE), 161);
    }

    #[test]
    pub fn part2_example() {
        const SAMPLE: &str = indoc! {"
            xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
        "};
        assert_eq!(part2(SAMPLE), 48);
    }
}
