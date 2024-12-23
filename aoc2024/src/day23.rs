use aoc_runner_derive::aoc;
use itertools::Itertools;

struct Graph {
    nodes: rustc_hash::FxHashMap<usize, smallvec::SmallVec<[usize; 16]>>,
    connection_map: [[bool; Self::SIZE]; Self::SIZE],
}

impl Graph {
    const OFFSET: usize = (b'z' - b'a' + 1) as usize;
    const SIZE: usize = Self::OFFSET * Self::OFFSET;

    #[inline]
    fn parse(input: &str) -> Self {
        let mut connection_map = [[false; Self::SIZE]; Self::SIZE];
        let nodes: rustc_hash::FxHashMap<_, smallvec::SmallVec<_>> = input
            .as_bytes()
            .chunks(6)
            .fold(rustc_hash::FxHashMap::default(), |mut acc, connection| {
                let nodes = unsafe {
                    [
                        Self::OFFSET
                            .unchecked_mul((connection[0].unchecked_sub(b'a')) as usize)
                            .unchecked_add((connection[1].unchecked_sub(b'a')) as usize),
                        Self::OFFSET
                            .unchecked_mul((connection[3].unchecked_sub(b'a')) as usize)
                            .unchecked_add((connection[4].unchecked_sub(b'a')) as usize),
                    ]
                };
                connection_map[nodes[0]][nodes[1]] = true;
                connection_map[nodes[1]][nodes[0]] = true;
                acc.entry(nodes[0]).or_default().push(nodes[1]);
                acc.entry(nodes[1]).or_default().push(nodes[0]);
                acc
            });
        Self {
            nodes,
            connection_map,
        }
    }
}

#[aoc(day23, part1)]
#[must_use]
pub fn part1(input: &str) -> usize {
    const T_RANGE: std::ops::Range<usize> =
        (Graph::OFFSET * (b't' - b'a') as usize)..(Graph::OFFSET * (b't' - b'a' + 1) as usize);
    let graph = Graph::parse(input);
    let mut visited = [false; Graph::SIZE];
    T_RANGE
        .filter_map(|node_a| {
            if let Some(connected_nodes) = graph.nodes.get(&node_a) {
                visited[node_a] = true;
                Some(
                    connected_nodes
                        .iter()
                        .enumerate()
                        .map(|(i, &node_b)| {
                            connected_nodes
                                .iter()
                                .skip(i)
                                .filter(|&&node_c| {
                                    graph.connection_map[node_b][node_c]
                                        && !visited[node_b]
                                        && !visited[node_c]
                                })
                                .count()
                        })
                        .sum::<usize>(),
                )
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day23, part2)]
#[must_use]
pub fn part2(input: &str) -> String {
    const EXPECTED_LEN: usize = 14;
    let graph = Graph::parse(input);
    graph
        .nodes
        .into_iter()
        .fold(
            (
                smallvec::SmallVec::<[_; EXPECTED_LEN]>::new(),
                [false; Graph::SIZE],
            ),
            |(max, mut visited), (node_a, connected_nodes)| {
                if visited[node_a] {
                    return (max, visited);
                }
                let current = std::iter::once(node_a).chain(connected_nodes).fold(
                    smallvec::SmallVec::<[_; EXPECTED_LEN]>::new(),
                    |mut current, node_b| {
                        if current.iter().all(|&c| graph.connection_map[node_b][c]) {
                            visited[node_b] = true;
                            current.push(node_b);
                        }
                        current
                    },
                );
                if current.len() > max.len() {
                    (current, visited)
                } else {
                    (max, visited)
                }
            },
        )
        .0
        .into_iter()
        .sorted_unstable()
        .map(|node| {
            format!(
                "{}{}",
                char::from(((node / Graph::OFFSET) as u8) + b'a'),
                char::from(((node % Graph::OFFSET) as u8) + b'a')
            )
        })
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        kh-tc
        qp-kh
        de-cg
        ka-co
        yn-aq
        qp-ub
        cg-tb
        vc-aq
        tb-ka
        wh-tc
        yn-cg
        kh-ub
        ta-co
        de-co
        tc-td
        tb-wq
        wh-td
        ta-ka
        td-qp
        aq-cg
        wq-ub
        ub-vc
        de-ta
        wq-aq
        wq-vc
        wh-yn
        ka-de
        kh-ta
        co-tc
        wh-qp
        tb-vc
        td-yn
    "};

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(SAMPLE), 7);
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(part2(SAMPLE), "co,de,ka,ta");
    }
}
