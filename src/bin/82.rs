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

use rayon::prelude::*;
use shared::a_star::{self, Coord, Matrix};

fn main() {
    let m = build_matrix();
    let result = minimal_path_sum(&m);

    println!("{}", result);
}

fn minimal_path_sum(matrix: &Problem82Matrix) -> u32 {
    (0..matrix.height())
        .into_par_iter()
        .map(|start_y| {
            let start = Coord { x: 0, y: start_y };
            let path = a_star::a_star(matrix, start);

            matrix.path_sum(path)
        })
        .min()
        .unwrap()
}

#[derive(Debug)]
struct Problem82Matrix {
    nodes: Vec<Vec<u32>>,
}

impl Matrix for Problem82Matrix {
    fn nodes(&self) -> &[Vec<u32>] {
        &self.nodes
    }

    fn neighbors(&self, node: Coord) -> Vec<Coord> {
        let Coord { x, y } = node;

        let up = Coord {
            x,
            y: y.wrapping_sub(1),
        };
        let right = Coord { x: x + 1, y };
        let down = Coord { x, y: y + 1 };

        let mut neighbors = vec![up, right, down];
        neighbors.retain(|neighbor| self.contains(*neighbor));

        neighbors
    }

    fn heuristic(&self, start: Coord) -> u32 {
        let right_margin = self.width() - 1;

        right_margin - start.x
    }

    fn reached_goal(&self, node: Coord) -> bool {
        node.x == self.width() - 1
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
    use super::Problem82Matrix;
    use shared::a_star::{self, Coord};

    #[test]
    fn test_example_path() {
        let matrix = demo_matrix();

        let start = Coord { x: 0, y: 1 };
        let path = a_star::a_star(&matrix, start);

        assert_eq!(
            path,
            vec![
                Coord { x: 0, y: 1 },
                Coord { x: 1, y: 1 },
                Coord { x: 2, y: 1 },
                Coord { x: 2, y: 0 },
                Coord { x: 3, y: 0 },
                Coord { x: 4, y: 0 },
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
