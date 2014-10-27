/* Problem 35: Circular primes
 *
 * The number, 197, is called a circular prime because all rotations of the digits: 197, 971, and
 * 719, are themselves prime.
 *
 * There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.
 *
 * How many circular primes are there below one million? */

extern crate shared;
use shared::{digits, sieve};

fn main() {
    let mut sieve = sieve::new::<uint>();
    let search_space: Vec<uint> = sieve.by_ref().take_while(|&prime| prime < 1_000_000).collect();

    let result = search_space.into_iter().filter(|&prime| {
        is_circular(prime, &mut sieve)
    }).count();

    println!("{}", result);
}

fn is_circular(prime: uint, sieve: &mut sieve::Sieve<uint>) -> bool {
    rotations_of(prime).into_iter().all(|rotation| {
        sieve.is_prime(rotation)
    })
}

fn rotations_of(number: uint) -> Vec<uint> {
    use std::num::pow;

    let number_count = digits::new(number).count();

    let biggest_unit = pow(10u, number_count - 1);

    range(0, number_count).scan(number, |state, _| {
        let prev_rotation = *state;

        let head = prev_rotation % 10;
        let tail = prev_rotation / 10;

        let rotation = head * biggest_unit + tail;
        *state = rotation;

        Some(rotation)
    }).collect()
}
