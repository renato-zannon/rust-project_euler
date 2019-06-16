/* Problem 81: Path sum: two ways
 *
 * In the 5 by 5 matrix below, the minimal path sum from the top left to the bottom right, by only
 * moving to the right and downonly moving to the right and down, is indicated in bold red and is
 * equal to 2427.
 *
 * 131*  673   234   103   18
 *
 * 201*  96*   342*  965   150
 *
 * 630   803   746*  422*  111
 *
 * 537   699   497   121*  956
 *
 * 805   732   524   37*   331*
 *
 * Find the minimal path sum, in matrix.txt, a 31K text file containing a 80 by 80 matrix, from the
 * top left to the bottom right by only moving right and down.
 **/

use std::collections::{HashMap, HashSet};

fn main() {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Coord(u32, u32);

trait AStar {
    fn heuristic(start: Coord, end: Coord) -> u32;
    fn dist_between(start: Coord, end: Coord) -> u32;
    fn neighbors(node: Coord) -> Vec<Coord>;

    fn a_star(start: Coord, goal: Coord) -> Vec<Coord> {
        let mut closed_set = HashSet::<Coord>::new();

        let mut open_set = HashSet::new();
        open_set.insert(start);

        let mut came_from = HashMap::new();

        let mut g_score = HashMap::<Coord, u32>::new();
        g_score.insert(start, 0);

        let mut f_score = HashMap::new();
        f_score.insert(start, Self::heuristic(start, goal));

        while let Some(current) = Self::lowest_score(&open_set, &f_score) {
            if current == goal {
                return reconstruct_path(&came_from, goal);
            }

            open_set.remove(&current);
            closed_set.insert(current);

            for neighbor in Self::neighbors(current) {
                if closed_set.contains(&neighbor) {
                    continue;
                }

                let tentative_gscore = get_score(&g_score, &current)
                    .saturating_add(Self::dist_between(current, neighbor));

                if !open_set.contains(&neighbor) {
                    open_set.insert(neighbor);
                } else if tentative_gscore >= get_score(&g_score, &neighbor) {
                    continue;
                }

                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_gscore);
                f_score.insert(
                    neighbor,
                    g_score[&neighbor].saturating_add(Self::heuristic(neighbor, goal)),
                );
            }
        }

        panic!("There's no path from start to goal");
    }

    fn lowest_score(set: &HashSet<Coord>, scores: &HashMap<Coord, u32>) -> Option<Coord> {
        set.iter().min_by_key(|c| get_score(scores, c)).cloned()
    }
}

fn get_score(scores: &HashMap<Coord, u32>, node: &Coord) -> u32 {
    scores.get(node).cloned().unwrap_or(std::u32::MAX)
}

fn reconstruct_path(came_from: &HashMap<Coord, Coord>, start: Coord) -> Vec<Coord> {
    let mut path = vec![start];
    let mut current = start;

    while came_from.contains_key(&current) {
        current = came_from[&current];
        path.push(current);
    }

    path
}
