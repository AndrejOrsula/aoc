use aoc_runner_derive::aoc;

const OFFSET: usize = 11;
const LEN: usize = OFFSET * OFFSET;

#[aoc(day21, part1)]
#[must_use]
pub fn part1(input: &str) -> u32 {
    #[rustfmt::skip]
    const LUT: [u32; LEN] = [
        00,18,26,21,12,27,22,13,28,23,14,
        10,00,25,12,19,26,13,20,27,14,21,
        22,21,00,10,11,12,19,20,13,20,21,
        17,16,18,00,10,21,12,19,22,13,20,
        16,21,19,18,00,22,21,12,23,22,13,
        23,22,16,17,18,00,10,11,12,19,20,
        18,17,21,16,17,18,00,10,21,12,19,
        17,22,22,21,16,19,18,00,22,21,12,
        24,23,17,18,19,16,17,18,00,10,11,
        19,18,22,17,18,21,16,17,18,00,10,
        18,23,23,22,17,22,21,16,19,18,00,
    ];
    let mut nums = [0; LEN];
    input.as_bytes().chunks(5).for_each(|line| {
        let num = line[..3].iter().fold(0, |acc, b| unsafe {
            10_u32
                .unchecked_mul(acc)
                .unchecked_add(u32::from(b.unchecked_sub(b'0')))
        });
        let mut tab: usize = 0;
        line[..3].iter().for_each(|b| {
            let shift = unsafe { b.unchecked_sub(b'/') as usize };
            nums[unsafe { tab.unchecked_add(shift) }] += num;
            tab = unsafe { OFFSET.unchecked_mul(shift) };
        });
        nums[tab] += num;
    });
    (0..LEN)
        .map(|i| unsafe { nums.get_unchecked(i).unchecked_mul(*LUT.get_unchecked(i)) })
        .sum()
}

#[aoc(day21, part2)]
#[must_use]
pub fn part2(input: &str) -> u64 {
    #[rustfmt::skip]
    const LUT: [u64; LEN] = [
        0x000000000,0x537CD85F4,0x750C8265A,0x68E25F039,0x36F530AAC,0x750C8265B,0x68E25F03A,0x36F530AAD,0x750C8265C,0x68E25F03B,0x36F530AAE,
        0x353A0A244,0x000000000,0x750C82659,0x36F530AAC,0x59C3B603D,0x750C8265A,0x36F530AAD,0x59C3B603E,0x750C8265B,0x36F530AAE,0x59C3B603F,
        0x64C7A35D4,0x64C7A35D3,0x000000000,0x353A0A244,0x353A0A245,0x36F530AAC,0x59C3B603D,0x59C3B603E,0x36F530AAD,0x59C3B603E,0x59C3B603F,
        0x54DAE1BCB,0x4D734A4AE,0x537CD85F4,0x000000000,0x353A0A244,0x68E25F039,0x36F530AAC,0x59C3B603D,0x68E25F03A,0x36F530AAD,0x59C3B603E,
        0x4D734A4AE,0x66E727CB5,0x537CD85F5,0x537CD85F4,0x000000000,0x68E25F03A,0x68E25F039,0x36F530AAC,0x68E25F03B,0x68E25F03A,0x36F530AAD,
        0x64C7A35D5,0x64C7A35D4,0x4D734A4AE,0x54DAE1BCB,0x54DAE1BCC,0x000000000,0x353A0A244,0x353A0A245,0x36F530AAC,0x59C3B603D,0x59C3B603E,
        0x54DAE1BCC,0x4D734A4AF,0x66E727CB5,0x4D734A4AE,0x54DAE1BCB,0x537CD85F4,0x000000000,0x353A0A244,0x68E25F039,0x36F530AAC,0x59C3B603D,
        0x4D734A4AF,0x66E727CB6,0x66E727CB6,0x66E727CB5,0x4D734A4AE,0x537CD85F5,0x537CD85F4,0x000000000,0x68E25F03A,0x68E25F039,0x36F530AAC,
        0x64C7A35D6,0x64C7A35D5,0x4D734A4AF,0x54DAE1BCC,0x54DAE1BCD,0x4D734A4AE,0x54DAE1BCB,0x54DAE1BCC,0x000000000,0x353A0A244,0x353A0A245,
        0x54DAE1BCD,0x4D734A4B0,0x66E727CB6,0x4D734A4AF,0x54DAE1BCC,0x66E727CB5,0x4D734A4AE,0x54DAE1BCB,0x537CD85F4,0x000000000,0x353A0A244,
        0x4D734A4B0,0x66E727CB7,0x66E727CB7,0x66E727CB6,0x4D734A4AF,0x66E727CB6,0x66E727CB5,0x4D734A4AE,0x537CD85F5,0x537CD85F4,0x000000000,
    ];
    let mut nums = [0; LEN];
    input.as_bytes().chunks(5).for_each(|line| {
        let num = line[..3].iter().fold(0, |acc, b| unsafe {
            10_u64
                .unchecked_mul(acc)
                .unchecked_add(u64::from(b.unchecked_sub(b'0')))
        });
        let mut tab: usize = 0;
        line[..3].iter().for_each(|b| {
            let shift = unsafe { b.unchecked_sub(b'/') as usize };
            nums[unsafe { tab.unchecked_add(shift) }] += num;
            tab = unsafe { OFFSET.unchecked_mul(shift) };
        });
        nums[tab] += num;
    });
    (0..LEN)
        .map(|i| unsafe { nums.get_unchecked(i).unchecked_mul(*LUT.get_unchecked(i)) })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        029A
        980A
        179A
        456A
        379A
    "};

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(SAMPLE), 126_384);
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(part2(SAMPLE), 154_115_708_116_294);
    }
}
