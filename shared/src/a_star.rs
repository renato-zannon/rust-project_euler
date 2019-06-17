use std::hash::Hash;

mod algorithm;
pub mod matrix;

use self::algorithm::Algorithm;
pub use self::matrix::{Coord, Matrix};

pub trait Traversable {
    type Coord: Copy + Eq + Hash;

    fn heuristic(&self, start: Self::Coord) -> u32;
    fn dist_between(&self, start: Self::Coord, end: Self::Coord) -> u32;
    fn neighbors(&self, node: Self::Coord) -> Vec<Self::Coord>;
    fn reached_goal(&self, node: Self::Coord) -> bool;
}

pub fn a_star<T: Traversable>(graph: &T, start: T::Coord) -> Vec<T::Coord> {
    let astar = Algorithm::new(graph, start);
    astar.perform()
}
