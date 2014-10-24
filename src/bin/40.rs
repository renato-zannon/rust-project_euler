/* Problem 40: Champernowne's constant
 *
 * An irrational decimal fraction is created by concatenating the positive integers:
 *
 * 0.123456789101112131415161718192021...
 *
 * It can be seen that the 12th digit of the fractional part is 1.
 *
 * If dn represents the nth digit of the fractional part, find the value of the following
 * expression.
 *
 * d1 × d10 × d100 × d1000 × d10000 × d100000 × d1000000 */

extern crate shared;
use shared::digits;
use std::iter::{count, MultiplicativeIterator};

fn main() {
    let mut sequence = count(1u, 1)
        .flat_map(|number| digits::new(number))
        .enumerate();

    let get_digit = |number: uint| {
        let (_, digit) = sequence
            .find(|&(idx, _)| idx == number - 1)
            .unwrap();

        digit
    };

    let result = [1, 10, 100, 1000, 10000, 100000, 1000000]
        .iter()
        .map(|&position| get_digit(position))
        .product();

    println!("{}", result);
}