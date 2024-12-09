use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day9, part1)]
#[must_use]
pub fn part1(input: &str) -> usize {
    let mut memory = Vec::with_capacity(51200);

    let input = input.bytes().enumerate();
    let mut digits_rev = input
        .clone()
        .rev()
        .skip_while(|&(_, b)| !b.is_ascii_digit());
    let (mut i_rev, mut i_id_rev, mut b_rev, mut b_remaining_rev);
    (i_rev, b_rev) = digits_rev.next().unwrap();
    if i_rev % 2 == 1 {
        (i_rev, b_rev) = digits_rev.next().unwrap();
    }
    i_id_rev = i_rev / 2;
    b_remaining_rev = b_rev - b'0';

    for (i, b) in input {
        if i >= i_rev {
            for _ in 0..b_remaining_rev {
                memory.push(i_id_rev);
            }
            break;
        }
        if i % 2 == 0 {
            let i_id = i / 2;
            for _ in 0..(b - b'0') {
                memory.push(i_id);
            }
        } else {
            for _ in 0..(b - b'0') {
                if b_remaining_rev == 0 {
                    _ = digits_rev.next().unwrap();
                    (i_rev, b_rev) = digits_rev.next().unwrap();
                    if i >= i_rev {
                        break;
                    }
                    i_id_rev = i_rev / 2;
                    b_remaining_rev = b_rev - b'0';
                }
                b_remaining_rev -= 1;
                memory.push(i_id_rev);
            }
        }
    }
    memory.into_iter().enumerate().map(|(i, id)| i * id).sum()
}

#[aoc(day9, part2)]
#[must_use]
pub fn part2(input: &str) -> usize {
    let mut files = Vec::with_capacity(10000);
    input
        .bytes()
        .filter(|&b| b.is_ascii_digit())
        .enumerate()
        .fold(0, |mut start, (i, b)| {
            let len = b - b'0';
            if i % 2 == 0 {
                files.push(File {
                    id: i / 2,
                    start,
                    len,
                });
            }
            start += len as usize;
            start
        });

    (0..=files.last().unwrap().id).rev().for_each(|id| {
        let file_id = files.iter().position(|file| file.id == id).unwrap();
        if let Some(new_pos) = files
            .iter()
            .tuple_windows()
            .find_map(|(a, b)| {
                let end = a.start + a.len as usize;
                if end > files[file_id].start {
                    Some(None)
                } else if b.start - end >= files[file_id].len as usize {
                    Some(Some(end))
                } else {
                    None
                }
            })
            .flatten()
        {
            files[file_id].start = new_pos;
        }
        files.sort_by_key(|file| file.start);
    });

    files
        .into_iter()
        .map(|file| {
            (file.start..file.start + file.len as usize)
                .map(|idx| idx * file.id)
                .sum::<usize>()
        })
        .sum()
}

#[derive(Clone, Copy)]
struct File {
    id: usize,
    start: usize,
    len: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        2333133121414131402
    "};

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(SAMPLE), 1928);
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(part2(SAMPLE), 2858);
    }
}
