/* Problem 44: Pentagon numbers
 *
 * Pentagonal numbers are generated by the formula, Pn=n(3n−1)/2. The first ten pentagonal numbers
 * are:
 *
 * 1, 5, 12, 22, 35, 51, 70, 92, 117, 145, ...
 *
 * It can be seen that P4 + P7 = 22 + 70 = 92 = P8. However, their difference, 70 − 22 = 48, is not
 * pentagonal.
 *
 * Find the pair of pentagonal numbers, Pj and Pk, for which their sum and difference are pentagonal
 * and D = |Pk − Pj| is minimised; what is the value of D? */

extern crate num;

use std::iter::count;

fn main() {
  let mut found: Vec<uint> = Vec::new();

  for n in count(1u, 1) {
    let pent1 = calculate_pentagonal_for(n);

    for &pent2 in found.iter() {
      if is_pentagonal(pent1 - pent2) && is_pentagonal(pent1 + pent2) {
        println!("{}", pent1 - pent2);
        return;
      }
    }

    found.push(pent1);
  }
}

fn is_pentagonal(number: uint) -> bool {
  let delta = 1 + 24 * number;

  return take_sqrt(delta)
    .and_then(to_integer)
    .map(|integer_sqrt| (1 + integer_sqrt) % 6 == 0)
    .unwrap_or(false);

  fn take_sqrt(delta: uint) -> Option<f32> {
    delta.to_f32().map(|as_float| as_float.sqrt())
  }

  fn to_integer(delta_sqrt: f32) -> Option<uint> {
    let is_integer = delta_sqrt == delta_sqrt.floor();

    if is_integer {
      delta_sqrt.to_uint()
    } else {
      None
    }
  }
}

fn calculate_pentagonal_for(n: uint) -> uint {
  n * (3 * n - 1) / 2
}
