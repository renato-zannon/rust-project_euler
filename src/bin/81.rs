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

const MATRIX: &'static str = include_str!("../../data/81-matrix.txt");

use a_star::{AStar, Coord};

fn main() {
    let mut m = build_matrix();
    let result = m.minimal_path_sum();

    println!("{}", result);
}

fn build_matrix() -> Matrix {
    let nodes = MATRIX
        .lines()
        .map(|line| line.split(',').filter_map(|s| s.parse().ok()).collect())
        .collect();

    Matrix { nodes }
}

#[cfg(test)]
mod tests {
    use super::Matrix;
    use crate::a_star::{AStar, Coord};

    #[test]
    fn test_example_path() {
        let mut matrix = demo_matrix();

        let (start, end) = (matrix.top_left(), matrix.bottom_right());
        let path = matrix.a_star(start, end);

        assert_eq!(
            path,
            vec![
                Coord { x: 0, y: 0 },
                Coord { x: 0, y: 1 },
                Coord { x: 1, y: 1 },
                Coord { x: 2, y: 1 },
                Coord { x: 2, y: 2 },
                Coord { x: 3, y: 2 },
                Coord { x: 3, y: 3 },
                Coord { x: 3, y: 4 },
                Coord { x: 4, y: 4 },
            ]
        )
    }

    #[test]
    fn test_example_sum() {
        let mut matrix = demo_matrix();

        let result = matrix.minimal_path_sum();

        assert_eq!(result, 2427)
    }

    fn demo_matrix() -> Matrix {
        let nodes = vec![
            vec![131, 673, 234, 103, 18],
            vec![201, 96, 342, 965, 150],
            vec![630, 803, 746, 422, 111],
            vec![537, 699, 497, 121, 956],
            vec![805, 732, 524, 37, 331],
        ];

        Matrix { nodes }
    }
}

#[derive(Debug)]
struct Matrix {
    nodes: Vec<Vec<u32>>,
}

impl Matrix {
    fn minimal_path_sum(&mut self) -> u32 {
        let (start, end) = (self.top_left(), self.bottom_right());

        self.a_star(start, end)
            .into_iter()
            .map(|coord| self.nodes[coord.y as usize][coord.x as usize])
            .fold(0, |a, n| a + n)
    }

    fn width(&self) -> u32 {
        self.nodes[0].len() as u32
    }

    fn height(&self) -> u32 {
        self.nodes.len() as u32
    }

    fn top_left(&self) -> Coord {
        Coord { x: 0, y: 0 }
    }

    fn bottom_right(&self) -> Coord {
        Coord {
            x: self.width() - 1,
            y: self.height() - 1,
        }
    }

    fn contains(&self, coord: Coord) -> bool {
        coord.x < self.width() && coord.y < self.height()
    }
}

impl AStar for Matrix {
    fn dist_between(&mut self, start: Coord, end: Coord) -> u32 {
        let mut x = [start.x, end.x];
        x.sort_unstable();

        let mut y = [start.y, end.y];
        y.sort_unstable();

        assert!(x[1] - x[0] == 1 || y[1] - y[0] == 1);

        self.nodes[end.y as usize][end.x as usize]
    }

    fn neighbors(&mut self, node: Coord) -> Vec<Coord> {
        let Coord { x, y } = node;

        let right = Coord { x: x + 1, y };
        let down = Coord { x, y: y + 1 };

        let mut neighbors = vec![right, down];
        neighbors.retain(|neighbor| self.contains(*neighbor));

        neighbors
    }

    fn heuristic(&mut self, start: Coord, end: Coord) -> u32 {
        let (x1, y1) = (start.x as f32, start.y as f32);
        let (x2, y2) = (end.x as f32, end.y as f32);

        ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt() as u32
    }
}

mod a_star {
    use std::collections::{HashMap, HashSet};

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    pub struct Coord {
        pub x: u32,
        pub y: u32,
    }

    pub trait AStar {
        fn heuristic(&mut self, start: Coord, end: Coord) -> u32;
        fn dist_between(&mut self, start: Coord, end: Coord) -> u32;
        fn neighbors(&mut self, node: Coord) -> Vec<Coord>;

        fn a_star(&mut self, start: Coord, goal: Coord) -> Vec<Coord> {
            let mut closed_set = HashSet::<Coord>::new();

            let mut open_set = HashSet::new();
            open_set.insert(start);

            let mut came_from = HashMap::new();

            let mut g_score = HashMap::<Coord, u32>::new();
            g_score.insert(start, 0);

            let mut f_score = HashMap::new();
            f_score.insert(start, self.heuristic(start, goal));

            while let Some(current) = Self::lowest_score(&open_set, &f_score) {
                if current == goal {
                    return reconstruct_path(&came_from, goal);
                }

                open_set.remove(&current);
                closed_set.insert(current);

                for neighbor in self.neighbors(current) {
                    if closed_set.contains(&neighbor) {
                        continue;
                    }

                    let tentative_gscore = get_score(&g_score, &current)
                        .saturating_add(self.dist_between(current, neighbor));

                    if !open_set.contains(&neighbor) {
                        open_set.insert(neighbor);
                    } else if tentative_gscore >= get_score(&g_score, &neighbor) {
                        continue;
                    }

                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_gscore);
                    f_score.insert(
                        neighbor,
                        g_score[&neighbor].saturating_add(self.heuristic(neighbor, goal)),
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

        path.reverse();
        path
    }
}
