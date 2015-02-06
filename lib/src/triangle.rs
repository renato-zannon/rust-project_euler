use std::collections::BTreeMap;
use std::cell::RefCell;
use std::cmp::Eq;

#[derive(Eq, PartialEq, Debug)]
struct Value {
    coord: (u32, u32),
    value: u32,
    maximum: Option<u32>,
}

pub struct Triangle {
    values: BTreeMap<(u32, u32), RefCell<Value>>,
    height: u32,
}

pub fn new(raw: &[&[u32]]) -> Triangle {
    let mut values = BTreeMap::new();

    for (row_index, row) in raw.iter().enumerate() {
        assert_eq!(row.len(), row_index + 1);

        let row_index = row_index as u32;

        for (index, &raw_value) in row.iter().enumerate() {
            let coord = (row_index, index as u32);
            let value = Value { coord: coord, value: raw_value, maximum: None };

            values.insert(coord, RefCell::new(value));
        }
    }

    Triangle { values: values, height: raw.len() as u32 }
}

impl Triangle {
    pub fn maximum_total(&mut self) -> u32 {
        let origin_ref = self.get_coord((0, 0));

        {
            let mut origin = origin_ref.borrow_mut();
            origin.maximum = Some(origin.value);
        }

        self.update_maximums(origin_ref);

        self.values
            .iter()
            .filter_map(|(&(row, _), value)| {
                if row == self.height - 1 { Some(value) } else { None }
            })
            .max_by(|refvalue| refvalue.borrow().maximum)
            .and_then(|refmax| refmax.borrow().maximum)
            .unwrap()
    }

    fn update_maximums(&self, initial_ref: &RefCell<Value>) {
        let below = self.values_below(initial_ref.borrow().coord);

        match below {
            Some((left, right)) => {
                self.update_value(initial_ref, left);
                self.update_value(initial_ref, right);
            },

            None => return
        }
    }

    fn update_value(&self, current_ref: &RefCell<Value>, adjacent_ref: &RefCell<Value>) {
        let current_maximum = current_ref.borrow().maximum.unwrap();

        let (adj_max, new_maximum) = {
            let adjacent = adjacent_ref.borrow();
            let new_maximum = adjacent.value + current_maximum;

            let adj_max = match adjacent.maximum {
                Some(adj_max) => adj_max,
                None => 0
            };

            (adj_max, new_maximum)
        };

        if new_maximum > adj_max {
            adjacent_ref.borrow_mut().maximum = Some(new_maximum);
            self.update_maximums(adjacent_ref);
        }
    }

    fn get_coord(&self, coord: (u32, u32)) -> &RefCell<Value> {
        self.values.get(&coord).unwrap()
    }

    fn values_below(&self, (row, col): (u32, u32)) -> Option<(&RefCell<Value>, &RefCell<Value>)> {
        if row + 1 < self.height {
            let left  = self.get_coord((row + 1, col));
            let right = self.get_coord((row + 1, col + 1));

            Some((left, right))
        } else {
            None
        }
    }
}
