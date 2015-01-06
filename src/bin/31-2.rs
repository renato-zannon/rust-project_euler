/* Problem 31: Coin sums
 *
 * In England the currency is made up of pound, £, and pence, p, and there are eight coins in
 * general circulation:
 *
 *     1p, 2p, 5p, 10p, 20p, 50p, £1 (100p) and £2 (200p).
 *
 * It is possible to make £2 in the following way:
 *
 *     1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
 *
 * How many different ways can £2 be made using any number of coins? */

use std::iter::{repeat, range_inclusive};

const DENOMINATIONS: &'static [uint] = &[200, 100, 50, 20, 10, 5, 2, 1];

fn main() {
    println!("{}", ways_to_make(200, DENOMINATIONS));
}

// Dynamic programming solution, adapted from the overview PDF
fn ways_to_make(value: uint, denominations: &[uint]) -> uint {
    let mut ways: Vec<uint> = repeat(0).take(value + 1).collect();
    let ways = ways.as_mut_slice();

    ways[0] = 1;

    for &denom in denominations.iter() {
        for current in range_inclusive(denom, value) {
            ways[current] += ways[current - denom];
        }
    }

    *ways.last().unwrap()
}
