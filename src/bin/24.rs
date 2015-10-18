/* Problem 24: Lexicographic permutations
 *
 * A permutation is an ordered arrangement of objects. For example, 3124 is one possible permutation
 * of the digits 1, 2, 3 and 4. If all of the permutations are listed numerically or alphabetically,
 * we call it lexicographic order. The lexicographic permutations of 0, 1 and 2 are:
 *
 * 012   021   102   120   201   210
 *
 * What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9? */

extern crate itertools;
extern crate shared;

use itertools::Itertools;
use shared::Permutations;

fn main() {
    let digits = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let result = digits
        .permutations()
        .skip(1_000_000 - 1)
        .next()
        .unwrap();

    println!("{}", result.into_iter().join(""));
}
