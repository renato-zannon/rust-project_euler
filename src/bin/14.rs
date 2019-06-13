/* Problem 14: Longest Collatz sequence
 *
 * The following iterative sequence is defined for the set of positive integers:
 *
 * n → n/2 (n is even)
 * n → 3n + 1 (n is odd)
 *
 * Using the rule above and starting with 13, we generate the following sequence:
 * 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
 *
 * It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although
 * it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at
 * 1.
 *
 * Which starting number, under one million, produces the longest chain?
 *
 * NOTE: Once the chain starts the terms are allowed to go above one million. */

use num::Integer;
use rayon::prelude::*;

const MAX: u64 = 1_000_000;

fn main() {
    let result = (1..MAX)
        .into_par_iter()
        .map(|i| (i, collatz_length(i)))
        .max_by_key(|(_, length)| *length);

    println!("{}", result.unwrap().0);
}

fn collatz_length(number: u64) -> usize {
    let mut length = 1;
    let mut current_number = number;

    while current_number > 1 {
        current_number = if current_number.is_even() {
            current_number / 2
        } else {
            3 * current_number + 1
        };

        length += 1;
    }

    length
}
