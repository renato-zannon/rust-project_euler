/* Problem 7: 10001st prime
 *
 * By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see
 * that the 6th prime is 13.
 *
 * What is the 10 001st prime number? */

extern crate shared;

use shared::sieve::{self, Sieve};

fn main() {
    //let mut all_primes = count(1usize, 1).filter(naive_is_prime);
    //let mut all_primes = count(1usize, 1).filter(smart_is_prime);
    let mut all_primes: Sieve<u32> = sieve::new();

    let result = all_primes.nth(10_000).unwrap();
    println!("{}", result);
}

/* Naive implementation. Fast enough for 10 001 primes */
#[allow(dead_code)]
fn naive_is_prime(&num: &u32) -> bool {
    !(2..num).any(|divisor| num % divisor == 0)
}

/* From the overview pdf */
#[allow(dead_code)]
fn smart_is_prime(&num: &u32) -> bool {
    if num == 1 {
        false
    } else if num < 4 {
        true
    } else if num % 2 == 0 {
        false
    } else if num < 9 {
        true
    } else if num % 3 == 0 {
        false
    } else {
        let r = (num as f64).sqrt().ceil() as u32;

        (5..r + 1)
            .step_by(6)
            .all(|f| num % f != 0 && num % (f + 2) != 0)
    }
}
