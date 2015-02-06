/* Problem 41: Pandigital prime
 *
 * We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n
 * exactly once. For example, 2143 is a 4-digit pandigital and is also prime.
 *
 * What is the largest n-digit pandigital prime that exists? */

#![feature(collections)]

extern crate shared;
use shared::primes;

fn main() {
    let all_digits = [1u32, 2, 3, 4, 5, 6, 7, 8, 9];

    let permutations = (1us..9).flat_map(|end| {
        let digit_slice = &all_digits[..end];
        digit_slice.permutations()
    });

    let primes = permutations.filter_map(|permutation| {
        let number = to_number(&permutation[]);

        if primes::is_prime(number) {
            Some(number)
        } else {
            None
        }
    });

    println!("{}", primes.max().unwrap());
}

fn to_number(digits: &[u32]) -> u32 {
    digits.iter().fold(0, |acc, &digit| {
        acc * 10 + digit
    })
}
