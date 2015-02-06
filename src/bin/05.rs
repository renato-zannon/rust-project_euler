/* Problem 5: Smallest multiple
 *
 * 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any
 * remainder.
 *
 * What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20? */

#![feature(core)]
extern crate num;

use std::iter::range_inclusive as irange;
use num::integer::lcm;

fn main() {
    let n = irange(1u32, 20).fold(1, lcm);
    println!("{}", n);
}
