/* Problem 20: Factorial digit sum
 *
 * n! means n × (n − 1) × ... × 3 × 2 × 1
 *
 * For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
 * and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
 *
 * Find the sum of the digits in the number 100! */

#![feature(slicing_syntax)]

extern crate num;
use std::num::One;
use num::bigint::{BigUint, ToBigUint};

fn main() {
    let strnum = factorial(100).to_string();

    let result = strnum[].chars().fold(0, |digit, sum| digit + to_i(sum));
    println!("{}", result);
}

fn factorial(n: uint) -> BigUint {
    let mut result: BigUint = One::one();
    let mut remaining: uint = n;

    while remaining > 0 {
        result = result * remaining.to_biguint().unwrap();
        remaining -= 1;
    }

    result
}

fn to_i(chr: char) -> uint {
    chr.to_digit(10).unwrap()
}