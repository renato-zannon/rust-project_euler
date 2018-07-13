/* Problem 16: Power digit sum
 *
 * 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
 *
 * What is the sum of the digits of the number 2^1000? */

extern crate num;

use num::bigint::BigUint;
use num::{pow, FromPrimitive};

fn main() {
    let strnum = power_of_2(1_000).to_string();

    let result = strnum.chars().fold(0, |digit, sum| digit + to_i(sum));
    println!("{}", result);
}

fn power_of_2(power: usize) -> BigUint {
    let num: BigUint = FromPrimitive::from_u32(2).unwrap();
    pow(num, power)
}

fn to_i(chr: char) -> u32 {
    chr.to_digit(10).unwrap()
}
