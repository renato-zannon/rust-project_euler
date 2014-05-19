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
use std::iter::AdditiveIterator;

fn main() {
  let mut products = Vec::new();

  for x in range(1u, 10_000) {
    for y in range(1u, x) {
      let result = x * y;

      match is_9_pandigital(&[x, y, result]) {
        IsPandigital => {
          if !products.contains(&result) {
            products.push(result);
          }
        },

        TooLarge => break,
        _        => continue
      }
    }
  }

  println!("{}", products.move_iter().sum());
}

enum PandigitalResult {
  IsPandigital,
  TooSmall,
  TooLarge,
  HasRepetitions,
}

#[allow(dead_code)]
impl PandigitalResult {
  fn to_bool(self) -> bool {
    match self {
      IsPandigital => true,
      _            => false,
    }
  }
}

fn is_9_pandigital(numbers: &[uint]) -> PandigitalResult {
  let all_digits: Vec<uint> = numbers.iter().flat_map(|&number| {
    digits::new(number)
  }).collect();

  match all_digits.len().cmp(&9) {
    Less    => return TooSmall,
    Greater => return TooLarge,
    Equal   => (),
  }

  let mut found_numbers = Vec::from_elem(9, false);

  let only_uniques = all_digits.move_iter().all(|digit| {
    let found = match digit {
      0    => return false,
      1..9 => found_numbers.get_mut(digit - 1),
      _    => unreachable!(),
    };

    if *found {
      return false;
    } else {
      *found = true;
      return true;
    }
  });

  if only_uniques {
    IsPandigital
  } else {
    HasRepetitions
  }
}

#[cfg(test)]
mod tests {
  use super::is_9_pandigital;

  #[test]
  fn test_1_through_9() {
    assert!(is_9_pandigital(&[123, 456, 789]).to_bool());
  }

  #[test]
  fn test_out_of_order() {
    assert!(is_9_pandigital(&[135, 97, 28, 46]).to_bool());
  }

  #[test]
  fn test_not_all_numbers() {
    assert!(is_9_pandigital(&[123, 456, 7]).to_bool() == false);
  }

  #[test]
  fn test_with_repetitions() {
    assert!(is_9_pandigital(&[12, 3456, 7891]).to_bool() == false);
  }

  #[test]
  fn test_rejects_zeroes() {
    assert!(is_9_pandigital(&[135, 97002, 80004, 60]).to_bool() == false);
  }
}
