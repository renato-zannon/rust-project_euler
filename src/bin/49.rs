/* Problem 49: Prime permutations
 *
 * The arithmetic sequence, 1487, 4817, 8147, in which each of the terms increases by 3330, is
 * unusual in two ways: (i) each of the three terms are prime, and, (ii) each of the 4-digit numbers
 * are permutations of one another.
 *
 * There are no arithmetic sequences made up of three 1-, 2-, or 3-digit primes, exhibiting this
 * property, but there is one other 4-digit increasing sequence.
 *
 * What 12-digit number do you form by concatenating the three terms in this sequence? */

#![feature(slicing_syntax)]

extern crate shared;
use shared::{digits, sieve};

const MEMBER_GAP: uint = 3330;

fn main() {
    let mut sieve = sieve::new();

    let primes = sieve.clone()
        .skip_while(|&prime| prime < 999)
        .take_while(|&prime| prime <= 9999);

    let sequences = primes.filter_map(|prime| {
        let digits: Vec<uint> = digits::new(prime).rev().collect();

        let mut matching_permutations = digits[].permutations()
            .map(|digits| to_number(digits[]))
            .filter(|&perm| {
                perm > prime &&
                    (perm - prime) % MEMBER_GAP == 0 &&
                    sieve.is_prime(perm)
            });

        let mut sequence: Vec<uint> = matching_permutations.collect();
        sequence.push(prime);

        sequence.sort();
        sequence.dedup();

        if sequence.len() == 3 {
            Some(sequence)
        } else {
            None
        }
    });

    let sequence = sequences.filter(|seq| seq[0] != 1487).next().unwrap();

    let result = sequence.into_iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .concat();

    println!("{}", result);
}

fn to_number(digits: &[uint]) -> uint {
    digits.iter().fold(0, |result, &digit| {
        result * 10 + digit
    })
}