/* Problem 34: Digit factorials
 *
 * 145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.
 *
 * Find the sum of all numbers which are equal to the sum of the factorial of their digits.
 *
 * Note: as 1! = 1 and 2! = 2 are not sums they are not included. */

#![feature(core)]
extern crate shared;

use std::iter::{range_inclusive, AdditiveIterator, count};
use std::num::Int;
use shared::digits;

fn main() {
    let max_number = {
        let max_single_digit_factorial = factorial(9);

        let max_digits = count(1us, 1us).find(|&digits| {
            max_single_digit_factorial * digits < max_number_with_digits(digits)
        }).unwrap();

        max_single_digit_factorial * max_digits
    };

    let result = (3us..max_number).filter(|&number| {
        number_eqls_fact_sum(number)
    }).sum();

    println!("{}", result);
}

fn number_eqls_fact_sum(number: usize) -> bool {
    let mut fact_sum = 0;

    for digit in digits::new(number) {
        fact_sum += factorial(digit);

        if fact_sum > number {
            return false;
        }
    }

    fact_sum == number
}

fn factorial(n: usize) -> usize {
    range_inclusive(1, n).fold(1, |num, result| {
        num * result
    })
}

fn max_number_with_digits(digit_count: usize) -> usize {
    10us.pow(digit_count) - 1
}
