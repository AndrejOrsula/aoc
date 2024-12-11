use aoc_runner_derive::aoc;

#[aoc(day11, part1)]
#[must_use]
pub fn part1(input: &str) -> usize {
    solve(&mut parse_input(input), 25)
}

#[aoc(day11, part2)]
#[must_use]
pub fn part2(input: &str) -> usize {
    solve(&mut parse_input(input), 75)
}

fn parse_input(input: &str) -> rustc_hash::FxHashMap<usize, usize> {
    input
        .split_ascii_whitespace()
        .map(|s| (s.parse().unwrap(), 1))
        .collect()
}

fn solve(map: &mut rustc_hash::FxHashMap<usize, usize>, n_steps: u32) -> usize {
    (0..n_steps).for_each(|_| {
        map.clone()
            .iter()
            .filter(|(_, &value)| value != 0)
            .for_each(|(&key, &value)| {
                if key == 0 {
                    add(map, 1, value);
                    sub(map, key, value);
                } else {
                    let str_key = key.to_string();
                    if str_key.len() % 2 == 0 {
                        let (key0, key1) = {
                            let key_len_half = str_key.len() / 2;
                            (
                                str_key[..key_len_half].parse().unwrap(),
                                str_key[key_len_half..].parse().unwrap(),
                            )
                        };
                        add(map, key0, value);
                        add(map, key1, value);
                        sub(map, key, value);
                    } else {
                        add(map, 2024 * key, value);
                        sub(map, key, value);
                    }
                }
            });
    });
    map.values().sum()
}

fn add(map: &mut rustc_hash::FxHashMap<usize, usize>, key: usize, value: usize) {
    if let Some(key) = map.get_mut(&key) {
        *key += value;
    } else {
        map.insert(key, value);
    }
}

fn sub(map: &mut rustc_hash::FxHashMap<usize, usize>, key: usize, value: usize) {
    *map.get_mut(&key).unwrap() -= value;
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        125 17
    "};

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(SAMPLE), 55_312);
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(part2(SAMPLE), 65_601_038_650_482);
    }
}
