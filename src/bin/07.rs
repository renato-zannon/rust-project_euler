/* Problem 7: 10001st prime
 *
 * By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see
 * that the 6th prime is 13.
 *
 * What is the 10 001st prime number? */

#![allow(unused_imports)]

extern crate shared;

use std::iter::{count, range_step_inclusive};
use shared::sieve::{self, Sieve};
use std::num::{ToPrimitive, Float};

fn main() {
//let mut all_primes = count(1u, 1).filter(naive_is_prime);
//let mut all_primes = count(1u, 1).filter(smart_is_prime);
    let mut all_primes: Sieve<uint> = sieve::new();

    let result = all_primes.nth(10_000).unwrap();
    println!("{}", result);
}

/* Naive implementation. Fast enough for 10 001 primes */
#[allow(dead_code)]
fn naive_is_prime(&num: &uint) -> bool {
    !range(2, num).any(|divisor| num % divisor == 0)
}

/* From the overview pdf */
#[allow(dead_code)]
fn smart_is_prime(&num: &uint) -> bool {
    if num == 1           { false }
    else if num < 4       { true  }
    else if num % 2 == 0  { false }
    else if num < 9       { true  }
    else if num % 3 == 0  { false }
    else {
        let r = num.to_f64()
            .map(|as_float| as_float.sqrt())
            .and_then(|result| result.ceil().to_uint())
            .unwrap();

        range_step_inclusive(5, r, 6).all(|f| {
            num % f != 0 && num % (f + 2) != 0
        })
    }
}
