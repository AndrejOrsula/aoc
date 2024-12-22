use aoc_runner_derive::aoc;
use itertools::Itertools;

#[inline]
fn parse(input: &str) -> impl Iterator<Item = SecretNumber> + '_ {
    input
        .lines()
        .map(|line| {
            line.bytes().fold(0, |acc, b| unsafe {
                10_usize
                    .unchecked_mul(acc)
                    .unchecked_add(usize::from(b.unchecked_sub(b'0')))
            })
        })
        .map(SecretNumber)
}

#[aoc(day22, part1)]
#[must_use]
pub fn part1(input: &str) -> usize {
    parse(input)
        .map(|mut num| {
            (0..2000).for_each(|_| num.hash());
            num.0
        })
        .sum()
}

const BASE: usize = 18;
#[aoc(day22, part2)]
#[must_use]
pub fn part2(input: &str) -> u16 {
    const OFFSET: usize = 16;
    const SIZE: usize = BASE + BASE * OFFSET.pow(1) + BASE * OFFSET.pow(2) + BASE * OFFSET.pow(3);
    let mut map_value = [0_u16; SIZE];
    let mut map_visited = [u16::MAX; SIZE];
    for (i, mut num) in parse(input).enumerate() {
        let mut window: [usize; 4] = unsafe {
            std::iter::once(0)
                .chain(std::iter::from_fn(|| {
                    let prev = num;
                    num.hash();
                    Some(num.diff(prev))
                }))
                .take(4)
                .collect_vec()
                .try_into()
                .unwrap_unchecked()
        };
        (3..2000).for_each(|_| {
            let prev = num;
            num.hash();
            window = [window[1], window[2], window[3], num.diff(prev)];
            let index = (0..3).fold(window[3], |w, i| unsafe {
                OFFSET.unchecked_mul(w).unchecked_add(window[i])
            });
            if map_visited[index] != i as u16 {
                map_visited[index] = i as u16;
                map_value[index] += (num.0 % 10) as u16;
            }
        });
    }
    *map_value.iter().max().unwrap()
}

#[repr(transparent)]
#[derive(Clone, Copy)]
struct SecretNumber(usize);

impl SecretNumber {
    #[inline]
    fn hash(&mut self) {
        self.0 = ((self.0 << 6) ^ self.0) % (1 << 24);
        self.0 = ((self.0 >> 5) ^ self.0) % (1 << 24);
        self.0 = ((self.0 << 11) ^ self.0) % (1 << 24);
    }

    #[inline]
    fn diff(self, prev: Self) -> usize {
        unsafe {
            (self.0 % 10)
                .unchecked_add(BASE / 2)
                .unchecked_sub(prev.0 % 10)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    pub fn part1_example() {
        const SAMPLE: &str = indoc! {"
            1
            10
            100
            2024
        "};
        assert_eq!(part1(SAMPLE), 37_327_623);
    }

    #[test]
    pub fn part2_example() {
        const SAMPLE: &str = indoc! {"
            1
            2
            3
            2024
        "};
        assert_eq!(part2(SAMPLE), 23);
    }
}
