/* Problem 70: Totient permutation
 *
 * Euler's Totient function, φ(n) [sometimes called the phi function], is used to determine the number
 * of positive numbers less than or equal to n which are relatively prime to n. For example, as
 * 1, 2, 4, 5, 7, and 8, are all less than nine and relatively prime to nine, φ(9)=6.
 *
 * The number 1 is considered to be relatively prime to every positive number, so φ(1)=1.
 *
 * Interestingly, φ(87109)=79180, and it can be seen that 87109 is a permutation of 79180.
 *
 * Find the value of n, 1 < n < 107, for which φ(n) is a permutation of n and the ratio n/φ(n) produces a
 * minimum. */

use rayon;

use num::rational::Ratio;
use rayon::prelude::*;
use shared::{digits, sieve};

const MAX: u64 = 10_000_000;

#[derive(Debug)]
struct Cell {
    ratio: Ratio<u64>,
    value: u64,
}

fn main() {
    let sieve = sieve::new::<u64>();

    let primes: Vec<_> = sieve.take_while(|n| *n < MAX).collect();

    let mut totient_ratios: Vec<_> = (2..=MAX)
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

                if start >= chunk.len() {
                    continue;
                }

                let range = start..chunk.len();

                // Calculate n/phi(n) = product of (p / p - 1) for all prime divisors of n
                for cell in chunk[range].iter_mut().step_by(prime as usize) {
                    cell.ratio *= Ratio::new(prime, prime - 1);
                }
            }
        });

    let result = totient_ratios
        .into_par_iter()
        .filter(|cell| {
            let phi_ratio = cell.ratio.recip() * cell.value;

            digits_match(phi_ratio.to_integer(), cell.value)
        })
        .min_by_key(|cell| cell.ratio)
        .map(|cell| cell.value);

    println!("{:?}", result);
}

fn digits_match(n1: u64, n2: u64) -> bool {
    let (mut n1_digits, mut n2_digits): (Vec<u8>, Vec<u8>) =
        rayon::join(|| digits::new(n1).collect(), || digits::new(n2).collect());

    if n1_digits.len() != n2_digits.len() {
        return false;
    }

    rayon::join(
        || n1_digits.par_sort_unstable(),
        || n2_digits.par_sort_unstable(),
    );

    n1_digits == n2_digits
}
