use aoc_runner_derive::aoc;

fn parse(input: &str) -> Vec<String> {
    input.split(',').map(|x| x.trim().to_owned()).collect()
}

#[aoc(day15, part1)]
#[must_use]
pub fn part1(input: &str) -> u32 {
    let input = parse(input);
    input.iter().map(|step| u32::from(utils::hash(step))).sum()
}

#[aoc(day15, part2)]
#[must_use]
pub fn part2(input: &str) -> usize {
    let input = parse(input);
    let mut boxes = vec![smallvec::SmallVec::<[_; 5]>::new(); 256];
    for step in &input {
        let (label, focal_length) = step.split_once(['=', '-']).unwrap();
        let current_box = boxes.get_mut(utils::hash(label) as usize).unwrap();
        let slot_index = current_box.iter().position(|&(l, _)| l == label);
        if step.contains('=') {
            match slot_index {
                Some(i) => {
                    current_box[i] = (label, focal_length);
                }
                None => {
                    current_box.push((label, focal_length));
                }
            }
        } else if let Some(i) = slot_index {
            current_box.remove(i);
        }
    }

    boxes
        .iter()
        .enumerate()
        .flat_map(|(box_index, slot)| {
            slot.iter()
                .enumerate()
                .map(move |(slot_index, &(_, focal_length))| {
                    let focal_length: usize = focal_length.parse().unwrap();
                    (box_index + 1) * (slot_index + 1) * focal_length
                })
        })
        .sum()
}

mod utils {
    pub fn hash(input: &str) -> u8 {
        input
            .bytes()
            .fold(0, |acc, byte| acc.wrapping_add(byte).wrapping_mul(17))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
    "};

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(SAMPLE), 1320);
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(part2(SAMPLE), 145);
    }
}
