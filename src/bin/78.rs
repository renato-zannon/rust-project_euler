/* Problem 78: Coin partitions
 *
 * Let p() represent the number of different ways in which n coins can be separated into piles.
 * For example, five coins can be separated into piles in exactly seven different ways, so p(5)=7.
 *
 * OOOOO
 * OOOO O
 * OOO OO
 * OOO O O
 * OO OO O
 * OO O O O
 * O O O O O
 *
 * Find the least value of n for which p() is divisible by one million.
 **/

use rayon::prelude::*;

const COMPUTE_UNTIL: i32 = 60_000;
const TARGET: i32 = 1_000_000;

fn main() {
    let mut counts = vec![0; COMPUTE_UNTIL as usize];
    counts[0] = 1;

    for piles in 1..COMPUTE_UNTIL {
        let piles = piles as usize;

        for index in piles..counts.len() {
            counts[index] = (counts[index] + counts[index - piles]) % TARGET;
        }
    }

    let result = counts
        .into_par_iter()
        .enumerate()
        .find_first(|(_index, n)| *n == 0);

    println!("{}", result.unwrap().0);
}
