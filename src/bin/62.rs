/* Problem 62: Cubic permutations
 *
 * The cube, 41063625 (345³), can be permuted to produce two other cubes: 56623104 (384³) and
 * 66430125 (405³). In fact, 41063625 is the smallest cube which has exactly three permutations of
 * its digits which are also cube.
 *
 * Find the smallest cube for which exactly five permutations of its digits are cube. */

extern crate shared;

use shared::digits;

use std::collections::{TreeMap, TreeSet};
use std::iter::count;

const PERMUTATION_COUNT: uint = 5;

fn main() {
    let mut map: TreeMap<Vec<u8>, TreeSet<u64>> = TreeMap::new();

    for base in count(2, 1) {
        let cube = base * base * base;

        let cube_digits = {
            let mut ord: Vec<u8> = digits::new(cube).rev().collect();
            ord.sort();
            ord
        };

        let set: &mut TreeSet<u64> = {
            if !map.contains_key(&cube_digits) {
                map.insert(cube_digits.clone(), TreeSet::new());
            }

            &mut map[cube_digits]
        };
        set.insert(cube);

        if set.len() == PERMUTATION_COUNT {
            println!("{}", set.iter().next().unwrap());
            return;
        }
    }
}