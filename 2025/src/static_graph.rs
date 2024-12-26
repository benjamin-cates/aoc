use std::{
    cmp::Reverse,
    collections::{BTreeMap, HashMap, HashSet, VecDeque},
    hash::Hash,
};

use priority_queue::PriorityQueue;

pub trait Nodeable: Hash + Clone + Eq + PartialOrd + Ord {}
impl<T> Nodeable for T where T: Hash + Clone + Eq + PartialOrd + Ord {}

pub struct StaticGraph<Node: Nodeable> {
    pub adjacency_lists: HashMap<Node, BTreeMap<Node, i64>>,
    pub nodes_set: HashSet<Node>,
}

impl<Node: Nodeable> StaticGraph<Node> {
    /// Get the edge weight from a to b
    pub fn get_weight(&self, a: &Node, b: &Node) -> Option<i64> {
        self.adjacency_lists.get(a)?.get(b).copied()
    }
    /// Returns true if edge from a to b exists
    pub fn has_edge(&self, a: &Node, b: &Node) -> bool {
        self.get_weight(a, b).is_some()
    }
    /// Create an empty graph
    pub fn new() -> Self {
        StaticGraph {
            adjacency_lists: HashMap::new(),
            nodes_set: HashSet::new(),
        }
    }
    /// Add edges as undirected (bidirectional)
    pub fn add_undirected_edges<I>(self, edges: I) -> Self
    where
        I: IntoIterator<Item = (Node, Node)>,
    {
        self.add_undirected_weighted_edges(edges.into_iter().map(|(n1, n2)| (n1, n2, 1)))
    }
    /// Add edges as undirected (bidirectional) with edge weights
    pub fn add_undirected_weighted_edges<I>(mut self, edges: I) -> Self
    where
        I: IntoIterator<Item = (Node, Node, i64)>,
    {
        for edge in edges {
            self.adjacency_lists
                .entry(edge.0.clone())
                .or_default()
                .insert(edge.1.clone(), edge.2);
            self.adjacency_lists
                .entry(edge.1.clone())
                .or_default()
                .insert(edge.0.clone(), edge.2);
            self.nodes_set.insert(edge.0.clone());
            self.nodes_set.insert(edge.1.clone());
        }
        self
    }
    /// Add edges as undirected (bidirectional)
    pub fn add_directed_edges<I>(self, edges: I) -> Self
    where
        I: IntoIterator<Item = (Node, Node)>,
    {
        self.add_directed_weighted_edges(edges.into_iter().map(|(n1, n2)| (n1, n2, 1)))
    }
    /// Add edges as undirected (bidirectional) with edge weights
    pub fn add_directed_weighted_edges<I>(mut self, edges: I) -> Self
    where
        I: IntoIterator<Item = (Node, Node, i64)>,
    {
        for edge in edges {
            self.adjacency_lists
                .entry(edge.0.clone())
                .or_default()
                .insert(edge.1.clone(), edge.2);
            self.nodes_set.insert(edge.0.clone());
            self.nodes_set.insert(edge.1.clone());
        }
        self
    }
    /// Takes the graph with weighted edges and a source node and returns the shortest path to any node that satisfies the target predicate.
    pub fn dijkstras<'a>(
        &'a self,
        source: &'a Node,
        target: impl Fn(&'a Node) -> bool,
    ) -> Option<Vec<Node>> {
        let mut queue: PriorityQueue<&Node, Reverse<i64>> = PriorityQueue::new();
        let mut back_list: HashMap<&Node, &Node> = HashMap::new();
        for node in self.nodes_set.iter() {
            queue.push(node, Reverse(i64::MAX));
        }
        queue.change_priority(source, Reverse(0));
        while !queue.is_empty() {
            let (cur, cost) = queue.pop().unwrap();
            if cost.0 == i64::MAX {
                return None;
            }
            if target(cur) {
                let mut path: Vec<Node> = vec![cur.clone()];
                while let Some(next) = back_list.get(path.last().unwrap()) {
                    path.push((*next).clone());
                }
                path.reverse();
                return Some(path);
            }
            for (neighbor, dist) in self.adjacency_lists.get(cur).unwrap() {
                if let Some((_, neighbor_priority)) = queue.get(neighbor) {
                    if neighbor_priority.0 > cost.0 + dist {
                        queue.change_priority(neighbor, Reverse(cost.0 + dist));
                        back_list.insert(neighbor, cur);
                    }
                }
            }
        }
        return None;
    }
    pub fn dijkstras_set<'a>(
        &'a self,
        source: &'a Node,
        target: impl Fn(&'a Node) -> bool,
    ) -> Option<HashSet<&'a Node>> {
        let mut queue: PriorityQueue<&Node, Reverse<i64>> = PriorityQueue::new();
        let mut back_list: HashMap<&Node, HashSet<&Node>> = HashMap::new();
        for node in self.nodes_set.iter() {
            queue.push(node, Reverse(i64::MAX));
        }
        queue.change_priority(source, Reverse(0));
        while !queue.is_empty() {
            let (cur, cost) = queue.pop().unwrap();
            if cost.0 == i64::MAX {
                return None;
            }
            if target(cur) {
                return back_list.get(cur).cloned();
            }
            for (neighbor, dist) in self.adjacency_lists.get(cur).unwrap() {
                if let Some((_, neighbor_priority)) = queue.get(neighbor) {
                    if neighbor_priority.0 == cost.0 + dist {
                        if back_list.get(cur).is_some() {
                            let to_insert = back_list.get(cur).unwrap().clone();
                            back_list.entry(neighbor).or_default().extend(to_insert);
                            back_list.entry(neighbor).or_default().insert(cur);
                        }
                    }
                    if neighbor_priority.0 > cost.0 + dist {
                        queue.change_priority(neighbor, Reverse(cost.0 + dist));
                        let hash_set_b4 = match back_list.get(cur) {
                            Some(b4) => b4.clone(),
                            None => HashSet::new(),
                        };
                        back_list.entry(neighbor).insert_entry(hash_set_b4);
                        back_list.entry(neighbor).or_default().insert(cur);
                    }
                }
            }
        }
        return None;
    }
}
impl<'a, Node: Nodeable> StaticGraph<Node> {
    pub fn into_dfs_iter(&'a self, source: &'a Node) -> DfsIter<'a, Node> {
        DfsIter::<Node> {
            graph: self,
            stack: Vec::from([source]),
            discovered_set: HashSet::from([source]),
        }
    }
    pub fn into_bfs_iter(&'a self, source: &'a Node) -> BfsIter<'a, Node> {
        BfsIter::<Node> {
            graph: self,
            queue: VecDeque::from([source]),
            discovered_set: HashSet::from([source]),
        }
    }
}
pub struct BfsIter<'a, Node: Nodeable> {
    graph: &'a StaticGraph<Node>,
    queue: VecDeque<&'a Node>,
    discovered_set: HashSet<&'a Node>,
}

impl<'a, Node: Nodeable> Iterator for BfsIter<'a, Node> {
    type Item = &'a Node;
    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.is_empty() {
            return None;
        }
        let next = self.queue.pop_front().unwrap();
        for (neighbor, _) in self.graph.adjacency_lists.get(next).unwrap().iter() {
            // Add to the queue if it is not already in the visited set
            if self.discovered_set.insert(neighbor) {
                self.queue.push_back(neighbor);
            }
        }
        Some(next)
    }
}

pub struct DfsIter<'a, Node: Nodeable> {
    graph: &'a StaticGraph<Node>,
    stack: Vec<&'a Node>,
    discovered_set: HashSet<&'a Node>,
}

impl<'a, Node: Nodeable> Iterator for DfsIter<'a, Node> {
    type Item = &'a Node;
    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            return None;
        }
        let next = self.stack.pop().unwrap();
        for (neighbor, _) in self.graph.adjacency_lists.get(next).unwrap().iter() {
            // Add to the queue if it is not already in the visited set
            if self.discovered_set.insert(neighbor) {
                self.stack.push(neighbor);
            }
        }
        Some(next)
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::StaticGraph;

    #[test]
    fn test_wfs() {
        let graph = StaticGraph::new().add_undirected_edges([
            (0, 1),
            (1, 2),
            (2, 3),
            (2, 4),
            (2, 5),
            (4, 6),
        ]);
        let iter = graph.into_bfs_iter(&0);
        assert_eq!(
            iter.cloned().collect::<Vec<i32>>(),
            vec![0, 1, 2, 3, 4, 5, 6]
        );
        let iter = graph.into_dfs_iter(&0);
        assert_eq!(iter.cloned().collect::<Vec<_>>(), vec![0, 1, 2, 5, 4, 6, 3]);
    }

    #[test]
    fn test_dijkstras() {
        let graph = StaticGraph::new().add_undirected_weighted_edges([(0, 2, 1)]);
        assert_eq!(graph.dijkstras(&0, |n| *n == 2), Some(vec![0, 2]));
        let graph =
            StaticGraph::new().add_undirected_weighted_edges([(0, 1, 1), (1, 2, 1), (0, 2, 3)]);
        assert_eq!(graph.dijkstras(&0, |n| *n == 2), Some(vec![0, 1, 2]));
    }
}