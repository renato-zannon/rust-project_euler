/* Problem 83: Path sum: four ways
 *
 * NOTE: This problem is a significantly more challenging version of Problem 81.
 *
 * In the 5 by 5 matrix below, the minimal path sum from the top left to the bottom right, by
 * moving left, right, up, and down, is indicated in bold red and is equal to 2297.
 *
 * 131*  673   234*  103*  18*
 *
 * 201*  96*   342*  965   150*
 *
 * 630   803   746   422*  111*
 *
 * 537   699   497   121*  956
 *
 * 805   732   524   37*   331*
 *
 * Find the minimal path sum, in matrix.txt, a 31K text file containing a 80 by 80 matrix, from the
 * top left to the bottom right by moving left, right, up, and down.
 **/

const MATRIX: &'static str = include_str!("../../data/83-matrix.txt");

use shared::a_star::{Coord, Matrix};

fn main() {
    let m = build_matrix();
    let result = m.minimal_path_sum();

    println!("{}", result);
}

#[derive(Debug)]
struct Problem83Matrix {
    nodes: Vec<Vec<u32>>,
}

impl Matrix for Problem83Matrix {
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
        let left = Coord {
            x: x.wrapping_sub(1),
            y,
        };

        let mut neighbors = vec![up, left, right, down];
        neighbors.retain(|neighbor| self.contains(*neighbor));

        neighbors
    }
}

fn build_matrix() -> Problem83Matrix {
    let nodes = MATRIX
        .lines()
        .map(|line| line.split(',').filter_map(|s| s.parse().ok()).collect())
        .collect();

    Problem83Matrix { nodes }
}

#[cfg(test)]
mod tests {
    use super::Problem83Matrix;
    use shared::a_star::{self, Coord, Matrix};

    #[test]
    fn test_example_path() {
        let matrix = demo_matrix();

        let start = matrix.top_left();
        let path = a_star::a_star(&matrix, start);

        assert_eq!(
            path,
            vec![
                Coord { x: 0, y: 0 },
                Coord { x: 0, y: 1 },
                Coord { x: 1, y: 1 },
                Coord { x: 2, y: 1 },
                Coord { x: 2, y: 0 },
                Coord { x: 3, y: 0 },
                Coord { x: 4, y: 0 },
                Coord { x: 4, y: 1 },
                Coord { x: 4, y: 2 },
                Coord { x: 3, y: 2 },
                Coord { x: 3, y: 3 },
                Coord { x: 3, y: 4 },
                Coord { x: 4, y: 4 },
            ]
        )
    }

    #[test]
    fn test_example_sum() {
        let matrix = demo_matrix();

        let result = matrix.minimal_path_sum();

        assert_eq!(result, 2297)
    }

    fn demo_matrix() -> Problem83Matrix {
        let nodes = vec![
            vec![131, 673, 234, 103, 18],
            vec![201, 96, 342, 965, 150],
            vec![630, 803, 746, 422, 111],
            vec![537, 699, 497, 121, 956],
            vec![805, 732, 524, 37, 331],
        ];

        Problem83Matrix { nodes }
    }
}
