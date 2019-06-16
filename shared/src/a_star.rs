use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub mod matrix;
pub use matrix::{Coord, Matrix};

pub trait Traversable {
    type Coord: Copy + Eq + Hash;

    fn heuristic(&self, start: Self::Coord, end: Self::Coord) -> u32;
    fn dist_between(&self, start: Self::Coord, end: Self::Coord) -> u32;
    fn neighbors(&self, node: Self::Coord) -> Vec<Self::Coord>;
}

pub fn a_star<T: Traversable>(graph: &T, start: T::Coord, goal: T::Coord) -> Vec<T::Coord> {
    let mut closed_set = HashSet::new();

    let mut open_set = HashSet::new();
    open_set.insert(start);

    let mut came_from = HashMap::new();

    let mut g_score = HashMap::new();
    g_score.insert(start, 0);

    let mut f_score = HashMap::new();
    f_score.insert(start, graph.heuristic(start, goal));

    while let Some(current) = lowest_score::<T>(&open_set, &f_score) {
        if current == goal {
            return reconstruct_path::<T>(&came_from, goal);
        }

        open_set.remove(&current);
        closed_set.insert(current);

        for neighbor in graph.neighbors(current) {
            if closed_set.contains(&neighbor) {
                continue;
            }

            let tentative_gscore = get_score::<T>(&g_score, &current)
                .saturating_add(graph.dist_between(current, neighbor));

            if !open_set.contains(&neighbor) {
                open_set.insert(neighbor);
            } else if tentative_gscore >= get_score::<T>(&g_score, &neighbor) {
                continue;
            }

            came_from.insert(neighbor, current);
            g_score.insert(neighbor, tentative_gscore);
            f_score.insert(
                neighbor,
                g_score[&neighbor].saturating_add(graph.heuristic(neighbor, goal)),
            );
        }
    }

    panic!("There's no path from start to goal");
}

fn lowest_score<T: Traversable>(
    set: &HashSet<T::Coord>,
    scores: &HashMap<T::Coord, u32>,
) -> Option<T::Coord> {
    set.iter()
        .min_by_key(|c| get_score::<T>(scores, c))
        .cloned()
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
