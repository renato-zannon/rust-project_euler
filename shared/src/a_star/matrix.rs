use super::{a_star, Traversable};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: u32,
    pub y: u32,
}

pub trait Matrix: Sized {
    fn nodes(&self) -> &[Vec<u32>];
    fn neighbors(&self, node: Coord) -> Vec<Coord>;

    fn minimal_path_sum(&self) -> u32 {
        let (start, end) = (self.top_left(), self.bottom_right());
        let nodes = self.nodes();

        a_star(self, start, end)
            .into_iter()
            .map(|coord| nodes[coord.y as usize][coord.x as usize])
            .fold(0, |a, n| a + n)
    }

    fn width(&self) -> u32 {
        self.nodes()[0].len() as u32
    }

    fn height(&self) -> u32 {
        self.nodes().len() as u32
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

impl<M: Matrix> Traversable for M {
    type Coord = Coord;

    fn neighbors(&self, node: Coord) -> Vec<Coord> {
        <Self as Matrix>::neighbors(&self, node)
    }

    fn dist_between(&self, start: Coord, end: Coord) -> u32 {
        let mut x = [start.x, end.x];
        x.sort_unstable();

        let mut y = [start.y, end.y];
        y.sort_unstable();

        assert!(x[1] - x[0] == 1 || y[1] - y[0] == 1);

        self.nodes()[end.y as usize][end.x as usize]
    }

    fn heuristic(&self, start: Coord, end: Coord) -> u32 {
        let (x1, y1) = (start.x as f32, start.y as f32);
        let (x2, y2) = (end.x as f32, end.y as f32);

        ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt() as u32
    }
}
