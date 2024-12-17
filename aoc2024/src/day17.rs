use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day17, part1)]
#[must_use]
pub fn part1(input: &str) -> String {
    let mut prog = Program::parse(input);
    std::iter::from_fn(|| prog.step()).join(",")
}

#[aoc(day17, part2)]
#[must_use]
pub fn part2(input: &str) -> usize {
    Program::parse_instructions(input, 0)
        .instructions
        .into_iter()
        .rev()
        .fold(
            smallvec::SmallVec::<[usize; 32]>::from_elem(0, 1),
            |queue, instruction| {
                queue
                    .into_iter()
                    .flat_map(move |i| {
                        (0..8).filter_map(move |j| {
                            let reg_a = j | (i << 3);
                            Program::parse_instructions(input, reg_a)
                                .step()
                                .is_some_and(|i| i == instruction as usize)
                                .then_some(reg_a)
                        })
                    })
                    .collect()
            },
        )[0]
}

#[derive(Debug)]
struct Program {
    instructions: smallvec::SmallVec<[u8; 16]>,
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    pointer: usize,
}

impl Program {
    #[inline]
    fn parse(input: &str) -> Program {
        let (registers, instructions) = unsafe { input.split_once("\n\n").unwrap_unchecked() };
        let reg_a = registers
            .bytes()
            .skip(12)
            .take_while(u8::is_ascii_digit)
            .fold(0, |num, b| unsafe {
                10_usize
                    .unchecked_mul(num)
                    .unchecked_add(b.unchecked_sub(b'0') as usize)
            });
        let instructions = instructions
            .bytes()
            .skip(9)
            .step_by(2)
            .map(|b| unsafe { b.unchecked_sub(b'0') })
            .collect();

        Self {
            instructions,
            reg_a,
            reg_b: 0,
            reg_c: 0,
            pointer: 0,
        }
    }

    #[inline]
    fn parse_instructions(input: &str, reg_a: usize) -> Program {
        let (_, instructions) = unsafe { input.split_once("\n\n").unwrap_unchecked() };
        let instructions = instructions
            .bytes()
            .skip(9)
            .step_by(2)
            .map(|b| unsafe { b.unchecked_sub(b'0') })
            .collect();

        Self {
            instructions,
            reg_a,
            reg_b: 0,
            reg_c: 0,
            pointer: 0,
        }
    }

    #[inline]
    fn step(&mut self) -> Option<usize> {
        while self.pointer < self.instructions.len() {
            match self.instructions[self.pointer] {
                0 => {
                    self.reg_a >>= match self.instructions[unsafe { self.pointer.unchecked_add(1) }]
                    {
                        instruction @ 0..4 => instruction as usize,
                        4 => self.reg_a,
                        5 => self.reg_b,
                        6 => self.reg_c,
                        _ => unsafe { std::hint::unreachable_unchecked() },
                    }
                }
                1 => {
                    self.reg_b ^=
                        self.instructions[unsafe { self.pointer.unchecked_add(1) }] as usize;
                }
                2 => {
                    self.reg_b = match self.instructions[unsafe { self.pointer.unchecked_add(1) }] {
                        instruction @ 0..4 => instruction as usize,
                        4 => self.reg_a,
                        5 => self.reg_b,
                        6 => self.reg_c,
                        _ => unsafe { std::hint::unreachable_unchecked() },
                    } % 8;
                }
                3 => {
                    if self.reg_a != 0 {
                        self.pointer =
                            self.instructions[unsafe { self.pointer.unchecked_add(1) }] as usize;
                        continue;
                    }
                }
                4 => self.reg_b ^= self.reg_c,
                5 => {
                    let i = match self.instructions[unsafe { self.pointer.unchecked_add(1) }] {
                        instruction @ 0..4 => instruction as usize,
                        4 => self.reg_a,
                        5 => self.reg_b,
                        6 => self.reg_c,
                        _ => unsafe { std::hint::unreachable_unchecked() },
                    } % 8;
                    self.pointer = unsafe { self.pointer.unchecked_add(2) };
                    return Some(i);
                }
                6 => {
                    self.reg_b = self.reg_a
                        >> match self.instructions[unsafe { self.pointer.unchecked_add(1) }] {
                            instruction @ 0..4 => instruction as usize,
                            4 => self.reg_a,
                            5 => self.reg_b,
                            6 => self.reg_c,
                            _ => unsafe { std::hint::unreachable_unchecked() },
                        }
                }
                7 => {
                    self.reg_c = self.reg_a
                        >> match self.instructions[unsafe { self.pointer.unchecked_add(1) }] {
                            instruction @ 0..4 => instruction as usize,
                            4 => self.reg_a,
                            5 => self.reg_b,
                            6 => self.reg_c,
                            _ => unsafe { std::hint::unreachable_unchecked() },
                        }
                }
                _ => unsafe { std::hint::unreachable_unchecked() },
            }
            self.pointer = unsafe { self.pointer.unchecked_add(2) };
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE1: &str = indoc! {"
        Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0
    "};
    const SAMPLE2: &str = indoc! {"
        Register A: 2024
        Register B: 0
        Register C: 0

        Program: 0,3,5,4,3,0
    "};

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(SAMPLE1), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(part2(SAMPLE2), 117_440);
    }
}
