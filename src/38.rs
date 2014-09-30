/* Problem 38: Pandigital multiples
 *
 * Take the number 192 and multiply it by each of 1, 2, and 3:
 *
 *     192 × 1 = 192
 *     192 × 2 = 384
 *     192 × 3 = 576
 *
 * By concatenating each product we get the 1 to 9 pandigital, 192384576. We will call 192384576 the
 * concatenated product of 192 and (1,2,3)
 *
 * The same can be achieved by starting with 9 and multiplying by 1, 2, 3, 4, and 5, giving the
 * pandigital, 918273645, which is the concatenated product of 9 and (1,2,3,4,5).
 *
 * What is the largest 1 to 9 pandigital 9-digit number that can be formed as the concatenated
 * product of an integer with (1,2, ... , n) where n > 1? */

extern crate shared;
use shared::pandigital::{is_9_pandigital, IsPandigital, TooLarge};
use shared::digits;
use std::iter::{count, range_inclusive};

fn main() {
  let mut largest = 0u;

  for n in range(2u, 9) {
    for start in count(1u, 1) {
      let prod = concat_product(start, n);

      match is_9_pandigital(prod[]) {
        IsPandigital => {
          let num_prod = to_num(prod);

          if num_prod > largest {
            largest = num_prod;
          }
        },

        TooLarge => break,
        _        => (),
      }
    }
  }

  println!("{}", largest);
}

fn concat_product(number: uint, max_factor: uint) -> Vec<uint> {
  range_inclusive(1u, max_factor).map(|factor| {
    number * factor
  }).fold(Vec::with_capacity(9), |mut result, n| {
    result.extend(digits::new(n));
    result
  })
}

fn to_num(digits: Vec<uint>) -> uint {
  let mut multiplier = 1;
  let mut result = 0;

  for digit in digits.into_iter().rev() {
    result += digit * multiplier;
    multiplier *= 10;
  }

  result
}
