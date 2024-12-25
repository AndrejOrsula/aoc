use aoc_runner_derive::aoc;

#[aoc(day25, part1)]
#[must_use]
pub fn part1(input: &str) -> usize {
    let input = input
        .split("\n\n")
        .map(|block| {
            block
                .trim_ascii_end()
                .bytes()
                .fold(0, |acc, b| (b as usize & 1) | (acc << 1))
        })
        .collect::<smallvec::SmallVec<[_; 512]>>();
    input
        .iter()
        .enumerate()
        .map(|(i, k)| input.iter().skip(i + 1).filter(|&l| (k & l) == 0).count())
        .sum()
}

// Note: Day 25 does not have a part 2

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        #####
        .####
        .####
        .####
        .#.#.
        .#...
        .....

        #####
        ##.##
        .#.##
        ...##
        ...#.
        ...#.
        .....

        .....
        #....
        #....
        #...#
        #.#.#
        #.###
        #####

        .....
        .....
        #.#..
        ###..
        ###.#
        ###.#
        #####

        .....
        .....
        .....
        #....
        #.#..
        #.#.#
        #####
    "};

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(SAMPLE), 3);
    }
}
