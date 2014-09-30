/* Problem 32: Pandigital products
 *
 * We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n
 * exactly once; for example, the 5-digit number, 15234, is 1 through 5 pandigital.
 *
 * The product 7254 is unusual, as the identity, 39 × 186 = 7254, containing multiplicand,
 * multiplier, and product is 1 through 9 pandigital.
 *
 * Find the sum of all products whose multiplicand/multiplier/product identity can be written as a 1
 * through 9 pandigital.
 *
 * HINT: Some products can be obtained in more than one way so be sure to only include it once in
 * your sum. */

extern crate shared;
use shared::digits;
use shared::pandigital;
use shared::pandigital::{is_9_pandigital, PandigitalResult};
use std::iter::AdditiveIterator;

fn main() {
  let mut products = Vec::new();

  for x in range(1u, 10_000) {
    for y in range(1u, x) {
      let result = x * y;

      match pandigital_product(&[x, y, result]) {
        pandigital::IsPandigital => {
          if !products.contains(&result) {
            products.push(result);
          }
        },

        pandigital::TooLarge => break,
        _                    => continue
      }
    }
  }

  println!("{}", products.into_iter().sum());
}

fn pandigital_product(numbers: &[uint]) -> PandigitalResult {
  let all_digits: Vec<uint> = numbers.iter().flat_map(|&number| {
    digits::new(number)
  }).collect();

  is_9_pandigital(all_digits.as_slice())
}
