/* Problem 52: Permuted multiples
 *
 * It can be seen that the number, 125874, and its double, 251748, contain exactly the same digits,
 * but in a different order.
 *
 * Find the smallest positive integer, x, such that 2x, 3x, 4x, 5x, and 6x, contain the same
 * digits. */

#![feature(slicing_syntax)]

extern crate shared;
use shared::digits;
use std::iter::{range_inclusive, count};

fn main() {
    let result = count(1, 1).find(|&number| {
        let num_digits = digit_set(number);

        range_inclusive(2, 6).rev().all(|multiplier| {
            yields_same_digits(number * multiplier, &num_digits[])
        })
    }).unwrap();

    println!("{}", result);
}

fn yields_same_digits(number: usize, digits: &[bool]) -> bool {
    digits::new::<_, usize>(number).rev().all(|digit| digits[digit])
}

fn digit_set(number: usize) -> [bool; 10] {
    let mut found_numbers = [false; 10];
    for digit in digits::new::<_, usize>(number).rev() {
        found_numbers[digit] = true;
    }

    found_numbers
}
