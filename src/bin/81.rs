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

use shared::a_star::{Coord, Matrix};

fn main() {
    let m = build_matrix();
    let result = m.minimal_path_sum();

    println!("{}", result);
}

#[derive(Debug)]
struct Problem81Matrix {
    nodes: Vec<Vec<u32>>,
}

impl Matrix for Problem81Matrix {
    fn nodes(&self) -> &[Vec<u32>] {
        &self.nodes
    }

    fn neighbors(&self, node: Coord) -> Vec<Coord> {
        let Coord { x, y } = node;

        let right = Coord { x: x + 1, y };
        let down = Coord { x, y: y + 1 };

        let mut neighbors = vec![right, down];
        neighbors.retain(|neighbor| self.contains(*neighbor));

        neighbors
    }
}

fn build_matrix() -> Problem81Matrix {
    let nodes = MATRIX
        .lines()
        .map(|line| line.split(',').filter_map(|s| s.parse().ok()).collect())
        .collect();

    Problem81Matrix { nodes }
}

#[cfg(test)]
mod tests {
    use super::Problem81Matrix;
    use shared::a_star::{self, Coord, Matrix};

    #[test]
    fn test_example_path() {
        let matrix = demo_matrix();

        let (start, end) = (matrix.top_left(), matrix.bottom_right());
        let path = a_star::a_star(&matrix, start, end);

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
        let matrix = demo_matrix();

        let result = matrix.minimal_path_sum();

        assert_eq!(result, 2427)
    }

    fn demo_matrix() -> Problem81Matrix {
        let nodes = vec![
            vec![131, 673, 234, 103, 18],
            vec![201, 96, 342, 965, 150],
            vec![630, 803, 746, 422, 111],
            vec![537, 699, 497, 121, 956],
            vec![805, 732, 524, 37, 331],
        ];

        Problem81Matrix { nodes }
    }
}
