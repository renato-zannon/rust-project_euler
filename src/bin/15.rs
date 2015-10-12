/* Problem 15: Lattice Paths
 *
 * Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down,
 * there are exactly 6 routes to the bottom right corner.
 *
 * How many such routes are there through a 20×20 grid? */

use std::collections::HashMap;

const GRID_SIZE: u16 = 20;

fn main() {
    let result = ways_to_reach(GRID_SIZE, GRID_SIZE, &mut HashMap::new());
    println!("{}", result);
}

fn ways_to_reach(row: u16, column: u16, cache: &mut HashMap<(u16, u16), u64>) -> u64 {
    return match cache.get(&(row, column)) {
        Some(&value) => value,

        None => {
            let value = compute(row, column, cache);
            cache.insert((row, column), value);
            value
        }
    };
}

fn compute(row: u16, column: u16, cache: &mut HashMap<(u16, u16), u64>) -> u64 {
    match (row, column) {
        (_, 0)        => 1,
        (0, _)        => 1,
        (row, column) => ways_to_reach(row - 1, column, cache) + ways_to_reach(row, column - 1, cache)
    }
}
