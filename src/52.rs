/* Problem 52: Permuted multiples
 * 
 * It can be seen that the number, 125874, and its double, 251748, contain exactly the same digits,
 * but in a different order.
 * 
 * Find the smallest positive integer, x, such that 2x, 3x, 4x, 5x, and 6x, contain the same
 * digits. */

extern crate shared;
use shared::digits;
use std::collections::TreeSet;
use std::iter::{range_inclusive, count};

fn main() {
  let result = count(1u, 1u).find(|&number| {
    let num_digits = digit_set(number);

    range_inclusive(2u, 6u).rev().all(|multiplier| {
      num_digits == digit_set(number * multiplier)
    })
  });

  println!("{}", result);
}

fn digit_set(number: uint) -> TreeSet<uint> {
  digits::new(number).rev().collect()
}
