use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day25)]
fn parse(input: &str) -> petgraph::Graph<String, (), petgraph::Undirected> {
    let mut graph = petgraph::Graph::default();
    let mut nodes = std::collections::HashMap::new();
    input.lines().for_each(|line| {
        let (origin_node, target_nodes) = line.split_once(':').unwrap();
        let origin_node = nodes
            .entry(origin_node)
            .or_insert_with(|| graph.add_node(origin_node.to_owned()))
            .to_owned();
        target_nodes.split_whitespace().for_each(|node| {
            let node = nodes
                .entry(node)
                .or_insert_with(|| graph.add_node(node.to_owned()))
                .to_owned();
            graph.add_edge(node, origin_node, ());
        });
    });
    graph
}

#[aoc(day25, part1)]
fn part1(input: &petgraph::Graph<String, (), petgraph::Undirected>) -> usize {
    let min_cut_res: Result<_, ()> =
        rustworkx_core::connectivity::stoer_wagner_min_cut(&input, |_| Ok(1));
    let (min_cut, partition) = min_cut_res.unwrap().unwrap();
    assert_eq!(min_cut, 3);

    (input.node_count() - partition.len()) * partition.len()
}

// Note: Day 25 does not have a part 2

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn part1_example() {
        const SAMPLE: &str = indoc! {"
            jqt: rhn xhk nvd
            rsh: frs pzl lsr
            xhk: hfx
            cmg: qnr nvd lhk bvb
            rhn: xhk bvb hfx
            bvb: xhk hfx
            pzl: lsr hfx nvd
            qnr: nvd
            ntq: jqt hfx bvb xhk
            nvd: lhk
            lsr: lhk
            rzs: qnr cmg lsr rsh
            frs: qnr lhk lsr
        "};

        assert_eq!(part1(&parse(SAMPLE)), 54);
    }
}
