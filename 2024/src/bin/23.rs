use std::{
    collections::{BTreeSet, HashMap}, fmt::Debug, hash::Hash
};

use aoc24::StaticGraph;

fn main() {
    let input: &str = include_str!("../data/23.txt");
    let now = std::time::Instant::now();
    println!("Part 1: {} ({:?})", part1(input), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {} ({:?})", part2(input), now.elapsed());
}

// Finished in 06:10
fn part1(input: &str) -> usize {
    let edges = input
        .lines()
        .map(|v| (v.split("-").nth(0).unwrap(), v.split("-").nth(1).unwrap()));
    let graph = StaticGraph::new().add_undirected_edges(edges);
    let mut tris = 0;
    for node in graph.nodes_set.iter() {
        let neighbors = graph.adjacency_lists.get(node).unwrap();
        for (neighbor_1, _) in neighbors.iter() {
            for (neighbor_2, _) in neighbors.iter() {
                if neighbor_1 != neighbor_2 && graph.get_weight(neighbor_1, neighbor_2).is_some() {
                    if node.starts_with("t")
                        || neighbor_1.starts_with("t")
                        || neighbor_2.starts_with("t")
                    {
                        tris += 1;
                    }
                }
            }
        }
    }
    tris / 6
}

fn recursive_best_subgraph<T: Copy + Hash + Ord + Debug>(
    subgraph: BTreeSet<T>,
    graph: &HashMap<T, BTreeSet<T>>,
    mut to_consider: BTreeSet<T>,
    mut to_not_consider: BTreeSet<T>,
) -> BTreeSet<T> {
    // Remove branches early if they won't work
    if to_consider.len() + subgraph.len() <= 11 {
        return BTreeSet::new();
    }
    if to_consider.len() == 0 && to_not_consider.len() == 0 {
        return subgraph;
    }
    let mut best: BTreeSet<T> = BTreeSet::new();
    while to_consider.len() != 0 {
        let v = to_consider.first().unwrap().clone();
        let n = graph.get(&v).unwrap();
        let tried = recursive_best_subgraph(
            subgraph.union(&BTreeSet::from([v])).copied().collect(),
            graph,
            to_consider.intersection(n).copied().collect(),
            to_not_consider.intersection(n).copied().collect(),
        );
        if tried.len() > best.len() {
            best = tried;
        }
        to_not_consider.insert(v);
        to_consider.remove(&v);
    }
    best
}

// Finished in 51:02
fn part2(input: &str) -> String {
    let name_as_num = |str: &str| (((str.as_bytes()[0] as u16) << 8) + str.as_bytes()[1] as u16);
    let edges = input.lines().map(|v| {
        (
            name_as_num(v.split("-").nth(0).unwrap()),
            name_as_num(v.split("-").nth(1).unwrap()),
        )
    });
    let graph: StaticGraph<u16> = StaticGraph::new().add_undirected_edges(edges);
    let adjacency_lists = graph
        .adjacency_lists
        .iter()
        .map(|list| {
            (
                *list.0,
                list.1
                    .iter()
                    .map(|(k, _w)| k)
                    .cloned()
                    .collect::<BTreeSet<_>>(),
            )
        })
        .collect::<HashMap<u16, BTreeSet<u16>>>();
    let best = recursive_best_subgraph(
        BTreeSet::new(),
        &adjacency_lists,
        graph.nodes_set.iter().cloned().collect(),
        BTreeSet::new(),
    );
    let mut best = best
        .iter()
        .map(|v| {
            [(v >> 8) as u8 as char, (*v & 0xFF) as u8 as char]
                .iter()
                .collect::<String>()
        })
        .collect::<Vec<String>>();
    best.sort();
    best.join(",")
}

#[cfg(test)]
#[test]
fn test_example() {
    let input = "kh-tc
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
td-yn";
    assert_eq!(part1(input), 7);
    assert_eq!(part2(input).as_str(), "co,de,ka,ta");
}
