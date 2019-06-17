use crate::a_star;
use hashbrown::HashMap;

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
pub struct Value {
    coord: (u32, u32),
    value: u32,
}

pub struct Triangle {
    values: HashMap<(u32, u32), Value>,
    height: u32,
    biggest_node_value: u32,
}

impl a_star::Traversable for Triangle {
    type Coord = Value;

    fn heuristic(&self, start: Value) -> u32 {
        self.height - start.coord.0
    }

    // A* minimizes the distance traveled, but here we want to *maximize* it.
    //
    // To trick A* into doing this we invert our node values, using the biggest node value as "0",
    // so that nodes with bigger values are seen as being "closer" by the algorithm.
    fn dist_between(&self, _start: Value, end: Value) -> u32 {
        self.biggest_node_value - end.value
    }

    fn neighbors(&self, node: Value) -> Vec<Value> {
        let (row, col) = node.coord;

        if row + 1 >= self.height {
            return vec![];
        }

        let left = self.get_coord((row + 1, col));
        let right = self.get_coord((row + 1, col + 1));

        vec![left, right]
    }

    fn reached_goal(&self, node: Self::Coord) -> bool {
        node.coord.0 == self.height - 1
    }
}

pub fn new(raw: &[&[u32]]) -> Triangle {
    let mut values = HashMap::new();
    let mut biggest_node_value = 0;

    for (row_index, row) in raw.iter().enumerate() {
        assert_eq!(row.len(), row_index + 1);

        let row_index = row_index as u32;

        for (index, &raw_value) in row.iter().enumerate() {
            let coord = (row_index, index as u32);
            let value = Value {
                coord: coord,
                value: raw_value,
            };

            values.insert(coord, value);
            if raw_value > biggest_node_value {
                biggest_node_value = raw_value;
            }
        }
    }

    Triangle {
        height: raw.len() as u32,
        values,
        biggest_node_value,
    }
}

impl Triangle {
    pub fn maximum_total(&self) -> u32 {
        let path = a_star::a_star(self, self.get_coord((0, 0)));
        path.into_iter().map(|node| node.value).sum()
    }

    fn get_coord(&self, coord: (u32, u32)) -> Value {
        self.values.get(&coord).cloned().unwrap()
    }
}
