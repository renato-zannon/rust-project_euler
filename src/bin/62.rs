/* Problem 62: Cubic permutations
 *
 * The cube, 41063625 (345³), can be permuted to produce two other cubes: 56623104 (384³) and
 * 66430125 (405³). In fact, 41063625 is the smallest cube which has exactly three permutations of
 * its digits which are also cube.
 *
 * Find the smallest cube for which exactly five permutations of its digits are cube. */

#![feature(core)]
extern crate shared;

use shared::digits;

use std::collections::{BTreeMap, BTreeSet};
use std::iter::count;

const PERMUTATION_COUNT: usize = 5;

fn main() {
    let mut map: BTreeMap<Vec<u8>, BTreeSet<u64>> = BTreeMap::new();

    for base in count(2, 1) {
        let cube = base * base * base;

        let cube_digits = {
            let mut ord: Vec<u8> = digits::new(cube).rev().collect();
            ord.sort();
            ord
        };

        let set: &mut BTreeSet<u64> = {
            if !map.contains_key(&cube_digits) {
                map.insert(cube_digits.clone(), BTreeSet::new());
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
