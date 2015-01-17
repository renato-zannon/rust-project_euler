/* Problem 27: Quadratic primes
 *
 * Euler discovered the remarkable quadratic formula:
 *
 * n² + n + 41
 *
 * It turns out that the formula will produce 40 primes for the consecutive values n = 0 to 39.
 * However, when n = 40, 40² + 40 + 41 = 40(40 + 1) + 41 is divisible by 41, and certainly when n =
 * 41, 41² + 41 + 41 is clearly divisible by 41.
 *
 * The incredible formula n² − 79n + 1601 was discovered, which produces 80 primes for the
 * consecutive values n = 0 to 79. The product of the coefficients, −79 and 1601, is −126479.
 *
 * Considering quadratics of the form:
 *
 *     n² + an + b, where |a| < 1000 and |b| < 1000
 *
 *     where |n| is the modulus/absolute value of n
 *     e.g. |11| = 11 and |−4| = 4
 *
 * Find the product of the coefficients, a and b, for the quadratic expression that produces the
 * maximum number of primes for consecutive values of n, starting with n = 0. */

extern crate shared;

use shared::sieve;
use std::iter::count;

fn main() {
    let mut primes: sieve::Sieve<usize> = sieve::new();

    let mut max_product = 0;
    let mut max_prime_count = 0;

    // Make n = 0 in n² + an + b = p and you get b = p. So, unless b is a prime, the search will stop
    // on the very first iteration. Since I already have a eratosthenes sieve, I might as well use it
    // here to generate the possible values of b and speed things up a bit.
    let possible_bs: Vec<usize> = primes.by_ref().take_while(|&prime| prime < 1000).collect();

    for b in possible_bs.into_iter() {
        let b = b as int;

        for a in (-999i..999) {
            let prime_count = count(0i, 1).take_while(|&n| {
                let value = n * n + a * n + b;
                if value < 0 { return false; }

                primes.is_prime(value as usize)
            }).count();

            if prime_count > max_prime_count {
                max_product = a * b;
                max_prime_count = prime_count;
            }
        }
    }

    println!("{}", max_product);
}
