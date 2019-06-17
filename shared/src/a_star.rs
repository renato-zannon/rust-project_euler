use hashbrown::{HashMap, HashSet};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::hash::Hash;

pub mod matrix;
pub use matrix::{Coord, Matrix};

pub trait Traversable {
    type Coord: Copy + Eq + Hash;

    fn heuristic(&self, start: Self::Coord) -> u32;
    fn dist_between(&self, start: Self::Coord, end: Self::Coord) -> u32;
    fn neighbors(&self, node: Self::Coord) -> Vec<Self::Coord>;
    fn reached_goal(&self, node: Self::Coord) -> bool;
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

pub fn a_star<T: Traversable>(graph: &T, start: T::Coord) -> Vec<T::Coord> {
    let mut closed_set = HashSet::new();

    let mut open_set = HashSet::new();
    open_set.insert(start);

    let mut open_set_queue = BinaryHeap::new();
    open_set_queue.push(scored(start, graph.heuristic(start)));

    let mut came_from = HashMap::new();

    let mut g_score = HashMap::new();
    g_score.insert(start, 0);

    while let Some(ScoredCoord { coord: current, .. }) = open_set_queue.pop() {
        if graph.reached_goal(current) {
            return reconstruct_path::<T>(&came_from, current);
        }

        open_set.remove(&current);
        closed_set.insert(current);

        for neighbor in graph.neighbors(current) {
            if closed_set.contains(&neighbor) {
                continue;
            }

            let tentative_gscore = get_score::<T>(&g_score, &current)
                .saturating_add(graph.dist_between(current, neighbor));

            if open_set.insert(neighbor) {
                let fscore = tentative_gscore.saturating_add(graph.heuristic(neighbor));
                open_set_queue.push(scored(neighbor, fscore));
            } else if tentative_gscore >= get_score::<T>(&g_score, &neighbor) {
                continue;
            }

            came_from.insert(neighbor, current);
            g_score.insert(neighbor, tentative_gscore);
        }
    }

    panic!("There's no path from start to goal");
}

fn get_score<T: Traversable>(scores: &HashMap<T::Coord, u32>, node: &T::Coord) -> u32 {
    scores.get(node).cloned().unwrap_or(std::u32::MAX)
}

fn reconstruct_path<T: Traversable>(
    came_from: &HashMap<T::Coord, T::Coord>,
    start: T::Coord,
) -> Vec<T::Coord> {
    let mut path = vec![start];
    let mut current = start;

    while came_from.contains_key(&current) {
        current = came_from[&current];
        path.push(current);
    }

    path.reverse();
    path
}
