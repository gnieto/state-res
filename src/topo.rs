use std::{
    cmp::{Ordering, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque},
    sync::Arc,
};
pub struct Node {
    incoming: Vec<usize>,
    outgoing: Vec<usize>,
}
impl Node {
    pub fn incoming(&self) -> usize {
        self.incoming.len()
    }
    pub fn outgoing(&self) -> usize {
        self.outgoing.len()
    }
}
pub struct OrdList<T> {
    items_idx: BTreeMap<T, usize>,
    nodes: Vec<Node>,
}
impl<T: Ord> OrdList<T> {
    pub fn add_edge(&mut self, from: &T, to: &T) -> Option<()> {
        let mut source = from;
        let mut target = to;
        let from_idx = *self.items_idx.get(source)?;
        let to_idx = *self.items_idx.get(target)?;
        let mut source_que = VecDeque::new();
        let mut source_stack = VecDeque::new();
        let mut target_que = VecDeque::new();
        let mut target_stack = VecDeque::new();
        // The number of incoming edqges of source
        let mut s_degree = self.nodes.get(from_idx)?.incoming();
        // The number of outgoing edges of our target (to) node
        let mut t_degree = self.nodes.get(to_idx)?.outgoing();
        // Discover which nodes need relabeling...
        while s_degree > t_degree {
            let d = s_degree.min(t_degree);
            s_degree -= d;
            t_degree -= d;
            if s_degree == 0 {
                for node in &self.nodes[from_idx..=to_idx] {
                    source_que.push_back(w);
                    source_stack.push_front(s);
                }
                if !source_que.is_empty() {
                    source = source_que.iter().max().unwrap();
                } else {
                    source = self.previous(to)
                }
                s_degree = self
                    .items_idx
                    .get(source)
                    .map(|idx| self.nodes.get(*idx))
                    .flatten()?
                    .incoming();
            }
            if t_degree == 0 {
                for node in &self.nodes[from_idx..=to_idx] {
                    target_que.push_back(w);
                    target_stack.push_front(s);
                }
                if !target_que.is_empty() {
                    target = target_que.iter().min().unwrap();
                } else {
                    target = self.next(from);
                }
                t_degree = self
                    .items_idx
                    .get(target)
                    .map(|idx| self.nodes.get(*idx))
                    .flatten()?
                    .outgoing();
            }
        }
        // Relabel the nodes...
        if source == to {
            source = self.previous(to);
        }
        while !source_stack.is_empty() {
            let s = source_stack.pop_front().unwrap();
            self.delete(s);
            self.insert_after(s, source);
            source = s;
        }
        if target == from {
            target = self.next(from);
        }
        while !target_stack.is_empty() {
            let t = target_stack.pop_back().unwrap();
            self.delete(t);
            self.insert_before(t, target);
            target = t;
        }
        Some(())
    }
    pub fn previous(&mut self, item: &T) -> &T {
        todo!()
    }
    pub fn next(&mut self, item: &T) -> &T {
        todo!()
    }
    pub fn delete(&mut self, item: &T) {
        todo!()
    }
    pub fn insert_after(&mut self, after: &T, item: &T) {
        todo!()
    }
    pub fn insert_before(&mut self, before: &T, item: &T) {
        todo!()
    }
}
