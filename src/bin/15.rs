/* Problem 15: Lattice Paths
 *
 * Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down,
 * there are exactly 6 routes to the bottom right corner.
 *
 * How many such routes are there through a 20×20 grid? */

use std::collections::TreeMap;

fn main() {
    let result = ways_to_reach(20, 20, &mut TreeMap::new());
    println!("{}", result);
}

fn ways_to_reach(row: uint, column: uint, cache: &mut TreeMap<(uint, uint), uint>) -> uint {
    return match cache.get(&(row, column)) {
        Some(&value) => value,

        None => {
            let value = compute(row, column, cache);
            cache.insert((row, column), value);
            value
        }
    };
}

fn compute(row: uint, column: uint, cache: &mut TreeMap<(uint, uint), uint>) -> uint {
    match (row, column) {
        (_, 0)        => 1,
        (0, _)        => 1,
        (row, column) => ways_to_reach(row - 1, column, cache) + ways_to_reach(row, column - 1, cache)
    }
}
