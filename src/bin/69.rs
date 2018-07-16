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

struct Cell {
    ratio: Ratio<u32>,
    value: u32,
}

fn main() {
    let sieve = sieve::new::<u32>();

    let primes: Vec<_> = sieve.take_while(|n| *n < MAX / 2).collect();

    let mut totient_ratios: Vec<_> = (1..=MAX)
        .map(|value| Cell {
            value,
            ratio: Ratio::from_integer(1),
        })
        .collect();

    totient_ratios
        .par_chunks_mut((MAX as usize) / 4)
        .for_each(|chunk| {
            let head_value = match chunk.first() {
                Some(head) => head.value,
                None => return,
            };

            for &prime in &primes {
                let start = {
                    let rem = head_value % prime;

                    if rem > 0 {
                        (prime - rem) as usize
                    } else {
                        0
                    }
                };

                if start > chunk.len() {
                    continue;
                }

                let range = start..chunk.len();

                // Calculate n/phi(n) = product of (p / p - 1) for all prime divisors of n
                for cell in chunk[range].iter_mut().step_by(prime as usize) {
                    cell.ratio *= Ratio::new(prime, prime - 1);
                }
            }
        });

    let result = totient_ratios.into_par_iter().max_by_key(|cell| cell.ratio);

    println!("{}", result.unwrap().value);
}
