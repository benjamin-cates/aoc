use std::{
    cmp::Reverse, collections::{HashMap, HashSet, VecDeque}, fmt::Debug, hash::Hash, marker::PhantomData
};

use priority_queue::PriorityQueue;

pub trait Nodeable: Hash + Clone + Eq + Debug {}
impl<T> Nodeable for T where T: Hash + Clone + Eq + Debug {}

pub struct LazyGraph<Node: Nodeable, Func, Iter>
where
    Func: Fn(&Node) -> Iter,
    Iter: Iterator<Item = (Node, i32)>,
{
    pub neighbors_func: Func,
    pd: PhantomData<Node>,
    pd2: PhantomData<Iter>,
}

impl<Node, Func, Iter> LazyGraph<Node, Func, Iter>
where
    Node: Nodeable,
    Func: Fn(&Node) -> Iter,
    Iter: Iterator<Item = (Node, i32)>,
{
    pub fn from_fn(func: Func) -> Self {
        Self {
            neighbors_func: func,
            pd: PhantomData::default(),
            pd2: PhantomData::default(),
        }
    }
    /// Get the edge weight from a to b
    pub fn get_weight(&self, a: &Node, b: &Node) -> Option<i32> {
        (self.neighbors_func)(a).find(|x| x.0 == *b).map(|v| v.1)
    }
    /// Get the edge weight from a to b
    /// Returns true if edge from a to b exists
    pub fn has_edge(&self, a: &Node, b: &Node) -> bool {
        (self.neighbors_func)(a).position(|x| x.0 == *b).is_some()
    }
    /// Takes the graph with weighted edges and a source node and returns the shortest path to any node that satisfies the target predicate.
    pub fn dijkstras<'a, 'b>(
        &'a self,
        source: &'a Node,
        target: impl Fn(Node) -> bool,
    ) -> Option<Vec<Node>> where Node: 'b{
        let mut queue: PriorityQueue<Node, Reverse<i32>> = PriorityQueue::new();
        let mut back_list: HashMap<Node, Node> = HashMap::new();
        queue.push(source.clone(), Reverse(0));
        while !queue.is_empty() {
            let (cur, cost) = queue.peek().unwrap();
            let (cur, cost) = (cur.clone(), cost.clone());
            queue.change_priority(&cur,Reverse(i32::MAX));
            if target(cur.clone()) {
                let mut path: Vec<Node> = vec![cur.clone()];
                while let Some(next) = back_list.get(path.last().unwrap()) {
                    path.push((*next).clone());
                }
                path.reverse();
                return Some(path);
            }
            for (neighbor, dist) in (self.neighbors_func)(&cur) {
                if let Some((_, neighbor_priority)) = queue.get(&neighbor) {
                    if neighbor_priority.0 > cost.0 + dist {
                        back_list.insert(neighbor.clone(), cur.clone());
                        queue.push_decrease(neighbor, Reverse(cost.0 + dist));
                    }
                } else {
                    if back_list.get(&neighbor).is_none() {
                        back_list.insert(neighbor.clone(), cur.clone());
                    }
                    queue.push_decrease(neighbor, Reverse(cost.0 + dist));
                }
            }
        }
        return None;
    }
    pub fn into_dfs_iter<'a>(&'a self, source: Node) -> DfsIter<'a, Node, Func, Iter> {
        DfsIter::<Node, Func, Iter> {
            graph: self,
            stack: Vec::from([source.clone()]),
            discovered_set: HashSet::from([source]),
        }
    }
    pub fn into_bfs_iter<'a>(&'a self, source: Node) -> BfsIter<'a, Node, Func, Iter> {
        BfsIter::<Node, Func, Iter> {
            graph: self,
            queue: VecDeque::from([source.clone()]),
            discovered_set: HashSet::from([source]),
        }
    }
}

pub struct BfsIter<'a, Node, Func, Iter>
where
    Node: Nodeable,
    Func: Fn(&Node) -> Iter,
    Iter: Iterator<Item = (Node, i32)>,
{
    graph: &'a LazyGraph<Node, Func, Iter>,
    queue: VecDeque<Node>,
    discovered_set: HashSet<Node>,
}

impl<'a, Node, Func, Iter> Iterator for BfsIter<'a, Node, Func, Iter>
where
    Node: Nodeable,
    Func: Fn(&Node) -> Iter,
    Iter: Iterator<Item = (Node, i32)>,
{
    type Item = Node;
    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.is_empty() {
            return None;
        }
        let next = self.queue.pop_front().unwrap();
        for (neighbor, _) in (self.graph.neighbors_func)(&next) {
            // Add to the queue if it is not already in the visited set
            if self.discovered_set.insert(neighbor.clone()) {
                self.queue.push_back(neighbor);
            }
        }
        Some(next)
    }
}

pub struct DfsIter<'a, Node, Func, Iter>
where
    Node: Nodeable,
    Func: Fn(&Node) -> Iter,
    Iter: Iterator<Item = (Node, i32)>,
{
    graph: &'a LazyGraph<Node, Func, Iter>,
    stack: Vec<Node>,
    discovered_set: HashSet<Node>,
}

impl<'a, Node, Func, Iter> Iterator for DfsIter<'a, Node, Func, Iter>
where
    Node: Nodeable,
    Func: Fn(&Node) -> Iter,
    Iter: Iterator<Item = (Node, i32)>,
{
    type Item = Node;
    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            return None;
        }
        let next = self.stack.pop().unwrap();
        for (neighbor, _) in (self.graph.neighbors_func)(&next) {
            // Add to the queue if it is not already in the visited set
            if self.discovered_set.insert(neighbor.clone()) {
                self.stack.push(neighbor);
            }
        }
        Some(next)
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::LazyGraph;

    #[test]
    fn test_wfs() {
        let graph = LazyGraph::<i32, _, _>::from_fn(|node| {
            ([(if *node < 6 { node + 1 } else { 0 }, 1)]).into_iter()
        });
        let iter = graph.into_bfs_iter(0);
        assert_eq!(
            iter.collect::<Vec<i32>>(),
            vec![0, 1, 2, 3, 4, 5, 6]
        );
        let iter = graph.into_dfs_iter(0);
        assert_eq!(iter.collect::<Vec<_>>(), vec![0, 1, 2, 5, 4, 6, 3]);
    }

}
