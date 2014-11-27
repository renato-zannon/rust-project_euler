/* Problem 37: Truncatable primes
 *
 * The number 3797 has an interesting property. Being prime itself, it is possible to continuously
 * remove digits from left to right, and remain prime at each stage: 3797, 797, 97, and 7. Similarly
 * we can work from right to left: 3797, 379, 37, and 3.
 *
 * Find the sum of the only eleven primes that are both truncatable from left to right and right to
 * left.
 *
 * NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes. */

extern crate shared;
use shared::{digits, sieve};
use std::iter::AdditiveIterator;

fn main() {
    let search_space = sieve::new::<uint>().skip_while(|&prime| prime < 10u);

    let mut prime_checker = sieve::new();

    let truncatable_primes = search_space
        .filter(|&prime| is_truncatable(prime, &mut prime_checker))
        .take(11);

    println!("{}", truncatable_primes.sum());
}

fn is_truncatable(prime: uint, sieve: &mut sieve::Sieve<uint>) -> bool {
    truncatable_from_left(prime, sieve) &&
        truncatable_from_right(prime, sieve)
}

fn truncatable_from_left(prime: uint, sieve: &mut sieve::Sieve<uint>) -> bool {
    let prime_digits = digits::new::<_, uint>(prime);

    prime_digits.rev().scan((0, 1), |state, digit| {
        let (previous, multiplier) = *state;
        let truncation = previous + digit * multiplier;

        *state = (truncation, multiplier * 10);
        Some(truncation)
    }).all(|truncation| {
        sieve.is_prime(truncation)
    })
}

fn truncatable_from_right(prime: uint, sieve: &mut sieve::Sieve<uint>) -> bool {
    let mut remaining = prime / 10;

    while remaining > 0 {
        if !sieve.is_prime(remaining) {
            return false;
        }

        remaining = remaining / 10;
    }

    return true;
}
