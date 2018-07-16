/* Problem 69: Totient maximum
 *
 * Euler's Totient function, φ(n) [sometimes called the phi function], is used to determine the number
 * of numbers less than n which are relatively prime to n. For example, as 1, 2, 4, 5, 7, and 8, are
 * all less than nine and relatively prime to nine, φ(9)=6.
 *
 * n   Relatively Prime  φ(n)  n/φ(n)
 * 2   1                 1     2
 * 3   1,2               2     1.5
 * 4   1,3               2     2
 * 5   1,2,3,4           4     1.25
 * 6   1,5               2     3
 * 7   1,2,3,4,5,6       6     1.1666...
 * 8   1,3,5,7           4     2
 * 9   1,2,4,5,7,8       6     1.5
 * 10  1,3,7,9           4     2.5
 *
 * It can be seen that n=6 produces a maximum n/φ(n) for n ≤ 10.
 *
 * Find the value of n ≤ 1,000,000 for which n/φ(n) is a maximum. */

extern crate num;
extern crate rayon;
extern crate shared;

use num::rational::Ratio;
use rayon::prelude::*;
use shared::sieve;
use std::iter;

const MAX: u32 = 1_000_000;

fn main() {
    let sieve = sieve::new::<u32>();

    let mut totient_ratios: Vec<_> = iter::repeat(Ratio::from_integer(1))
        .take(MAX as usize + 1)
        .collect();

    for prime in sieve {
        let start = prime * 2;
        if start > MAX {
            break;
        }

        let range = &mut totient_ratios[start as usize..];
        let uprime = prime as usize;

        range
            .into_par_iter()
            .enumerate()
            .for_each(move |(n, value)| {
                if (n as u32) % prime == 0 {
                    // calculate n / phi(n) directly
                    *value *= Ratio::new(prime, prime - 1);
                }
            });
    }

    let result = totient_ratios
        .into_par_iter()
        .enumerate()
        .max_by_key(|&(_, ratio)| ratio);

    println!("{}", result.unwrap().0);
}
