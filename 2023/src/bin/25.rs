use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input: &str = include_str!("../data/25.txt");
    println!("Answer to part1: {}", part1(input));
}

/// Parse undirected graph input and return a graph that is a map from node to a list of neighbors
fn parse_graph(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut conns: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let start = line.split(": ").nth(0).unwrap();
        if conns.get(start).is_none() {
            conns.insert(start, vec![]);
        }
        for other in line.split(": ").nth(1).unwrap().split(" ") {
            match conns.get_mut(other) {
                Some(vec) => {
                    vec.push(start);
                }
                None => {
                    conns.insert(other, vec![start]);
                }
            }
            conns.get_mut(start).unwrap().push(other);
        }
    }
    conns
}

/// Returns true if there is a valid depth first search from source to dest without using the
/// excluded edges. If there is a path, add all of those edges to the excluded set so they are not
/// used later.
fn depth_first_search<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    excluded: &mut HashSet<(&'a str, &'a str)>,
    source: &'a str,
    dest: &'a str,
) -> bool {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut stack: Vec<&str> = vec![];
    let mut back: HashMap<&str, &str> = HashMap::new();
    stack.push(source);
    while let Some(cur) = stack.pop() {
        if !visited.insert(cur) {
            continue;
        }
        for neighbor in graph.get(cur).unwrap().iter() {
            if visited.get(neighbor).is_some() {
                continue;
            }
            // Skip if current to neighbor is excluded
            if excluded.get(&(cur, *neighbor)).is_some()
                || excluded.get(&(*neighbor, cur)).is_some()
            {
                continue;
            }
            back.insert(neighbor, cur);
            stack.push(neighbor);
            if *neighbor == dest {
                let mut out: Vec<&str> = vec![];
                out.push(neighbor);
                let mut cur = *neighbor;
                while let Some(node) = back.get(cur) {
                    // Add edges used in this path to the excluded set so we do not use them in
                    // another traversal.
                    excluded.insert((node, cur));
                    cur = node;
                }
                return true;
            }
        }
    }
    // Stack is empty, no path found
    return false;
}

/// Determines if two nodes are for sure in the same set. Returns true if they have to be in the
/// same set or false if there is a possibility they aren't in the same set.
fn is_same_set(graph: &HashMap<&str, Vec<&str>>, source: &str, dest: &str) -> bool {
    let mut removed_edges: HashSet<(&str, &str)> = HashSet::new();
    let mut count = 0;
    while depth_first_search(graph, &mut removed_edges, source, dest) {
        count += 1;
        if count == 4 {
            return true;
        }
    }
    return false;
}

fn part1(input: &str) -> usize {
    let graph = parse_graph(input);
    let vertices: Vec<&str> = graph.keys().map(|x| *x).collect();
    let mut set1: Vec<&str> = Vec::new();
    // Add vertex 0 to the set
    set1.push(vertices[0]);
    let mut j = 0;
    while j < set1.len() {
        // Can safely break after 5 iterations via inspection
        if j == 5 {
            break;
        }

        for i in 1..vertices.len() {
            if set1.iter().find(|x| **x == vertices[i]).is_some() {
                continue;
            }
            // Test if vertices[i] is in the same set as set1[j]. If it is, add it to the set
            let same = is_same_set(&graph, set1[j], vertices[i]);
            if same {
                set1.push(vertices[i]);
            }
        }
        j += 1;
    }
    // Size of the set multiplied by size of the other set
    set1.len() * (graph.len() - set1.len())
}

#[allow(unused)]
fn part2(input: &str) -> usize {
    // There is no part 2 :)
    0
}

#[cfg(test)]
#[test]
fn test_solution() {
    let input = "jqt: rhn xhk nvd
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
frs: qnr lhk lsr";
    assert_eq!(part1(input), 54);
    assert_eq!(part2(input), 0);
}
