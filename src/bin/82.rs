/* Problem 82: Path sum: three ways
 *
 * NOTE: This problem is a more challenging version of Problem 81.
 *
 * The minimal path sum in the 5 by 5 matrix below, by starting in any cell in the left column and
 * finishing in any cell in the right column, and only moving up, down, and right, is indicated in
 * red and bold; the sum is equal to 994.
 *
 * 131   673   234*  103*  18*
 *
 * 201*  96*   342*  965   150
 *
 * 630   803   746   422   111
 *
 * 537   699   497   121   956
 *
 * 805   732   524   37    331
 *
 * Find the minimal path sum, in matrix.txt, a 31K text file containing a 80 by 80 matrix, from the
 * left column to the right column.
 **/

const MATRIX: &'static str = include_str!("../../data/82-matrix.txt");

use shared::a_star::{self, Traversable};

fn main() {
    let m = build_matrix();
    let result = minimal_path_sum(&m);

    println!("{}", result);
}

fn minimal_path_sum(matrix: &Problem82Matrix) -> u32 {
    let path = a_star::a_star(matrix, Coord::Start);
    path.into_iter()
        .filter_map(|node| match node {
            Coord::Start => None,
            Coord::Node { x, y } => Some(matrix.nodes[y as usize][x as usize]),
        })
        .sum()
}

#[derive(Debug)]
struct Problem82Matrix {
    nodes: Vec<Vec<u32>>,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum Coord {
    Start,
    Node { x: u32, y: u32 },
}

impl Problem82Matrix {
    fn nodes(&self) -> &[Vec<u32>] {
        &self.nodes
    }

    fn width(&self) -> u32 {
        self.nodes[0].len() as u32
    }

    fn height(&self) -> u32 {
        self.nodes().len() as u32
    }

    fn contains(&self, coord: Coord) -> bool {
        match coord {
            Coord::Start => true,
            Coord::Node { x, y } => x < self.width() && y < self.height(),
        }
    }

    fn first_column(&self) -> Vec<Coord> {
        (0..self.height())
            .into_iter()
            .map(|y| Coord::Node { x: 0, y })
            .collect()
    }
}

impl Traversable for Problem82Matrix {
    type Coord = Coord;

    fn neighbors(&self, node: Coord) -> Vec<Coord> {
        let (x, y) = match node {
            Coord::Start => return self.first_column(),
            Coord::Node { x, y } => (x, y),
        };

        let up = Coord::Node {
            x,
            y: y.wrapping_sub(1),
        };
        let right = Coord::Node { x: x + 1, y };
        let down = Coord::Node { x, y: y + 1 };

        let mut neighbors = vec![up, right, down];
        neighbors.retain(|neighbor| self.contains(*neighbor));

        neighbors
    }

    fn dist_between(&self, start: Coord, end: Coord) -> u32 {
        match (start, end) {
            (_, Coord::Start) => panic!("We shouldn't be trying to go to the start"),

            (Coord::Start, Coord::Node { x, y }) => {
                assert_eq!(x, 0);
                self.nodes[y as usize][0]
            }

            (Coord::Node { .. }, Coord::Node { x: x2, y: y2 }) => {
                self.nodes[y2 as usize][x2 as usize]
            }
        }
    }

    fn heuristic(&self, start: Coord) -> u32 {
        let right_margin = self.width() - 1;

        match start {
            Coord::Start => right_margin + 1,
            Coord::Node { x, .. } => right_margin - x,
        }
    }

    fn reached_goal(&self, node: Coord) -> bool {
        match node {
            Coord::Start => false,
            Coord::Node { x, .. } => x == self.width() - 1,
        }
    }
}

fn build_matrix() -> Problem82Matrix {
    let nodes = MATRIX
        .lines()
        .map(|line| line.split(',').filter_map(|s| s.parse().ok()).collect())
        .collect();

    Problem82Matrix { nodes }
}

#[cfg(test)]
mod tests {
    use super::{Coord, Problem82Matrix};
    use shared::a_star;

    #[test]
    fn test_example_path() {
        let matrix = demo_matrix();

        let path = a_star::a_star(&matrix, Coord::Start);

        assert_eq!(
            path,
            vec![
                Coord::Start,
                Coord::Node { x: 0, y: 1 },
                Coord::Node { x: 1, y: 1 },
                Coord::Node { x: 2, y: 1 },
                Coord::Node { x: 2, y: 0 },
                Coord::Node { x: 3, y: 0 },
                Coord::Node { x: 4, y: 0 },
            ]
        )
    }

    #[test]
    fn test_example_sum() {
        let matrix = demo_matrix();

        let result = super::minimal_path_sum(&matrix);

        assert_eq!(result, 994)
    }

    fn demo_matrix() -> Problem82Matrix {
        let nodes = vec![
            vec![131, 673, 234, 103, 18],
            vec![201, 96, 342, 965, 150],
            vec![630, 803, 746, 422, 111],
            vec![537, 699, 497, 121, 956],
            vec![805, 732, 524, 37, 331],
        ];

        Problem82Matrix { nodes }
    }
}
