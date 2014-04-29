/* Problem 5: Smallest multiple
 *
 * 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any
 * remainder.
 *
 * What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20? */

extern crate num;

use irange = std::iter::range_inclusive;
use num::lcm;

fn main() {
  let n = irange(1, 20).fold(1, lcm);

  println!("{}", n);
}
