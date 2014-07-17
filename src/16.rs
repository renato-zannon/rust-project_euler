/* Problem 16: Power digit sum
 *
 * 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
 *
 * What is the sum of the digits of the number 2^1000? */

extern crate num;

use std::num::FromPrimitive;
use std::num::pow;
use num::bigint::BigUint;

fn main() {
  let strnum = power_of_2(1_000).to_string();

  let result = strnum.as_slice().chars().fold(0, |digit, sum| digit + to_i(sum));
  println!("{}", result);
}

fn power_of_2(power: uint) -> BigUint {
  let num: BigUint = FromPrimitive::from_uint(2).unwrap();
  pow(num, power)
}

fn to_i(chr: char) -> uint {
  chr.to_digit(10).unwrap()
}
