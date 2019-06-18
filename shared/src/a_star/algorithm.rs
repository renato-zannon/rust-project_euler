use super::Traversable;
use hashbrown::{HashMap, HashSet};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub(super) struct Algorithm<'t, T: Traversable> {
    graph: &'t T,
    closed_set: HashSet<T::Coord>,
    open_set: HashSet<T::Coord>,
    open_set_queue: BinaryHeap<ScoredCoord<T::Coord>>,
    came_from: HashMap<T::Coord, T::Coord>,
    g_score: HashMap<T::Coord, u32>,
}

impl<'t, T: Traversable> Algorithm<'t, T> {
    pub(super) fn new(graph: &'t T, start: T::Coord) -> Self {
        let mut open_set = HashSet::new();
        open_set.insert(start);

        let mut open_set_queue = BinaryHeap::new();
        open_set_queue.push(scored(start, graph.heuristic(start)));

        let mut g_score = HashMap::new();
        g_score.insert(start, 0);

        Self {
            graph,
            open_set,
            open_set_queue,
            closed_set: HashSet::new(),
            came_from: HashMap::new(),
            g_score,
        }
    }

    pub(super) fn perform(mut self) -> Vec<T::Coord> {
        while let Some(current) = self.visit() {
            if self.graph.reached_goal(current) {
                return self.reconstruct_path(current);
            }

            let current_g_score = self.g_score(current);

            for neighbor in self.graph.neighbors(current) {
                if self.closed_set.contains(&neighbor) {
                    continue;
                }

                let distance = self.graph.dist_between(current, neighbor);
                let tentative_gscore = current_g_score.saturating_add(distance);

                let discovered = self.discover(neighbor, tentative_gscore);

                if discovered || tentative_gscore < self.g_score(neighbor) {
                    self.came_from.insert(neighbor, current);
                    self.g_score.insert(neighbor, tentative_gscore);
                }
            }
        }

        panic!("There's no path from start to goal");
    }

    fn visit(&mut self) -> Option<T::Coord> {
        let ScoredCoord { coord, .. } = self.open_set_queue.pop()?;

        self.open_set.remove(&coord);
        self.closed_set.insert(coord);

        Some(coord)
    }

    fn discover(&mut self, node: T::Coord, tentative_gscore: u32) -> bool {
        if !self.open_set.insert(node) {
            return false;
        }

        let fscore = tentative_gscore.saturating_add(self.graph.heuristic(node));
        self.open_set_queue.push(scored(node, fscore));
        true
    }

    fn g_score(&mut self, node: T::Coord) -> u32 {
        *self.g_score.entry(node).or_insert(std::u32::MAX)
    }

    fn reconstruct_path(&self, start: T::Coord) -> Vec<T::Coord> {
        let mut path = vec![start];
        let mut current = start;

        while self.came_from.contains_key(&current) {
            current = self.came_from[&current];
            path.push(current);
        }

        path.reverse();
        path
    }
}

struct ScoredCoord<C> {
    coord: C,
    score: u32,
}

impl<C> Ord for ScoredCoord<C> {
    fn cmp(&self, other: &Self) -> Ordering {
        // inverted on purpose, since we want the coordinates with the lowest f-score first
        other.score.cmp(&self.score)
    }
}

impl<C> PartialOrd for ScoredCoord<C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<C> PartialEq for ScoredCoord<C> {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl<C> Eq for ScoredCoord<C> {}

fn scored<C>(coord: C, score: u32) -> ScoredCoord<C> {
    ScoredCoord { coord, score }
}
