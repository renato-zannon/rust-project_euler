/* Problem 31: Coin sums
 *
 * In England the currency is made up of pound, £, and pence, p, and there are eight coins in
 * general circulation:
 *
 *     1p, 2p, 5p, 10p, 20p, 50p, £1 (100p) and £2 (200p).
 *
 * It is possible to make £2 in the following way:
 *
 *     1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
 *
 * How many different ways can £2 be made using any number of coins? */

use std::iter::{AdditiveIterator, range_step_inclusive};

static DENOMINATIONS: &'static [uint] = &[200, 100, 50, 20, 10, 5, 2, 1];

fn main() {
  println!("{}", ways_to_make(200, DENOMINATIONS));
}

fn ways_to_make(value: uint, denominations: &[uint]) -> uint {
  range(0, denominations.len()).map(|index| {
    let denomination = denominations[index];
    let remaining_denominations = denominations.slice_from(index + 1);

    if remaining_denominations.len() == 0 {
      return 1;
    }

    range_step_inclusive(denomination, value, denomination).map(|multiple| {
      let rest = value - multiple;

      if rest == 0 {
        1
      } else {
        ways_to_make(rest, remaining_denominations)
      }
    }).sum()
  }).sum()
}
