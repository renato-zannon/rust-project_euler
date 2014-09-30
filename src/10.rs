/* Problem 10: Summation of primes
 *
 * The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
 *
 * Find the sum of all the primes below two million. */

extern crate shared;
use shared::sieve;

static PRIME_MAX: uint = 2_000_000;

fn main() {
    let result = sieve::new()
        .take_while(|&prime| prime < PRIME_MAX)
        .fold(0u64, |acc, num| acc + (num as u64));

    println!("{}", result);
}
