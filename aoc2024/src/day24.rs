use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day24, part1)]
#[must_use]
pub fn part1(input: &str) -> usize {
    let (values, connections) = unsafe { input.split_once("\n\n").unwrap_unchecked() };
    let mut connections: smallvec::SmallVec<[_; 256]> = connections
        .lines()
        .map(|line| unsafe {
            let op = line.chars().nth(4).unwrap_unchecked();
            if op == 'O' {
                (
                    line.get_unchecked(0..3),
                    op,
                    line.get_unchecked(7..10),
                    line.get_unchecked(14..17),
                )
            } else {
                (
                    line.get_unchecked(0..3),
                    op,
                    line.get_unchecked(8..11),
                    line.get_unchecked(15..18),
                )
            }
        })
        .collect();
    let mut values = values
        .lines()
        .map(|line| {
            let (key, value) = (&line[..3], &line[5..]);
            (key, unsafe { value.parse::<usize>().unwrap_unchecked() })
        })
        .collect::<rustc_hash::FxHashMap<_, _>>();

    let mut seen = smallvec::SmallVec::<[bool; 256]>::from_elem(false, connections.len());
    while seen.iter().any(|&v| !v) {
        connections
            .iter_mut()
            .enumerate()
            .for_each(|(i, connection)| {
                if seen[i] {
                    return;
                }
                let (l, op, r, ret) = *connection;
                if let Some(l) = values.get(l) {
                    if let Some(r) = values.get(r) {
                        values.insert(
                            ret,
                            match op {
                                'A' => l & r,
                                'O' => l | r,
                                'X' => l ^ r,
                                _ => unsafe { std::hint::unreachable_unchecked() },
                            },
                        );
                        seen[i] = true;
                    }
                }
            });
    }

    (0..)
        .map_while(|i| values.get(format!("z{i:02}").as_str()).map(|&v| (v) << i))
        .sum()
}

#[aoc(day24, part2)]
#[must_use]
pub fn part2(input: &str) -> String {
    let (_, connections) = unsafe { input.split_once("\n\n").unwrap_unchecked() };
    let connections = connections.lines().map(|line| unsafe {
        let op = line.chars().nth(4).unwrap_unchecked();
        if op == 'O' {
            (
                line.get_unchecked(0..3),
                op,
                line.get_unchecked(7..10),
                line.get_unchecked(14..17),
            )
        } else {
            (
                line.get_unchecked(0..3),
                op,
                line.get_unchecked(8..11),
                line.get_unchecked(15..18),
            )
        }
    });
    let connection_cache = connections
        .clone()
        .flat_map(|(l, op, r, _)| [(l, op), (r, op)])
        .collect::<rustc_hash::FxHashSet<_>>();

    connections
        .filter_map(|(l, op, r, ret)| match op {
            'A' => {
                (l != "x00" && r != "x00" && !connection_cache.contains(&(ret, 'O'))).then_some(ret)
            }
            'X' => (((l.starts_with('x') || r.starts_with('x'))
                && (l != "x00" && r != "x00" && !connection_cache.contains(&(ret, 'X'))))
                || (!ret.starts_with('z') && !l.starts_with('x') && !r.starts_with('x')))
            .then_some(ret),
            'O' => (ret.starts_with('z') && ret != "z45").then_some(ret),
            _ => unsafe { std::hint::unreachable_unchecked() },
        })
        .sorted_unstable()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE1: &str = indoc! {"
        x00: 1
        x01: 1
        x02: 1
        y00: 0
        y01: 1
        y02: 0

        x00 AND y00 -> z00
        x01 XOR y01 -> z01
        x02 OR y02 -> z02
    "};

    const SAMPLE2: &str = indoc! {"
        x00: 1
        x01: 0
        x02: 1
        x03: 1
        x04: 0
        y00: 1
        y01: 1
        y02: 1
        y03: 1
        y04: 1

        ntg XOR fgs -> mjb
        y02 OR x01 -> tnw
        kwq OR kpj -> z05
        x00 OR x03 -> fst
        tgd XOR rvg -> z01
        vdt OR tnw -> bfw
        bfw AND frj -> z10
        ffh OR nrd -> bqk
        y00 AND y03 -> djm
        y03 OR y00 -> psh
        bqk OR frj -> z08
        tnw OR fst -> frj
        gnj AND tgd -> z11
        bfw XOR mjb -> z00
        x03 OR x00 -> vdt
        gnj AND wpb -> z02
        x04 AND y00 -> kjc
        djm OR pbm -> qhw
        nrd AND vdt -> hwm
        kjc AND fst -> rvg
        y04 OR y02 -> fgs
        y01 AND x02 -> pbm
        ntg OR kjc -> kwq
        psh XOR fgs -> tgd
        qhw XOR tgd -> z09
        pbm OR djm -> kpj
        x03 XOR y03 -> ffh
        x00 XOR y04 -> ntg
        bfw OR bqk -> z06
        nrd XOR fgs -> wpb
        frj XOR qhw -> z04
        bqk OR frj -> z07
        y03 OR x01 -> nrd
        hwm AND bqk -> z03
        tgd XOR rvg -> z12
        tnw OR pbm -> gnj
    "};

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(SAMPLE1), 4);
        assert_eq!(part1(SAMPLE2), 2024);
    }
}
