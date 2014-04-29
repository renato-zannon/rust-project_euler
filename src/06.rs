/* Problem 6: Sum square difference
 *
 * The sum of the squares of the first ten natural numbers is,
 * 1² + 2² + ... + 10² = 385
 *
 * The square of the sum of the first ten natural numbers is,
 * (1 + 2 + ... + 10)² = 55² = 3025
 *
 * Hence the difference between the sum of the squares of the first ten natural numbers and the
 * square of the sum is 3025 - 385 = 2640.
 *
 * Find the difference between the sum of the squares of the first one hundred natural numbers and
 * the square of the sum. */

extern crate shared;

use std::iter::AdditiveIterator;
use irange = std::iter::range_inclusive;

use shared::combinations;

// (a + b + c + ...)² = a² + b² + c² + ... + 2ab + 2ac + 2ad + 2bc...
// (a + b + c + ...)² - (a² + b² + c² + ...) = 2ab + 2ac + 2ad + 2bc...
fn main() {
  let first_natural_numbers: ~[uint] = irange(1u, 100u).collect();

  let result = combinations::new(first_natural_numbers)
    .filter(|&(a, b)| a != b) // No 2 * a * a
    .filter(|&(a, b)| a <  b) // No 2 * a * b + 2 * b * a
    .map(|(a, b)| 2u * a * b)
    .sum();

  println!("{}", result);
}
