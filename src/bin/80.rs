/* Problem 80: Square root digital expansion
 *
 * It is well known that if the square root of a natural number is not an integer, then it is
 * irrational. The decimal expansion of such square roots is infinite without any repeating pattern
 * at all.
 *
 * The square root of two is 1.41421356237309504880..., and the digital sum of the first one
 * hundred decimal digits is 475.
 *
 * For the first one hundred natural numbers, find the total of the digital sums of the first one
 * hundred decimal digits for all the irrational square roots.
 **/

use num::bigint::BigInt;
use num::traits::Zero;
use rayon::prelude::*;
use shared::digits;

fn main() {
    let total: i32 = (1..=100)
        .into_par_iter()
        .filter_map(|number| {
            let (sum, digit_count) = square_digits(number)
                .take(100)
                .fold((0, 0), |(sum, digit_count), digit| {
                    (sum + digit, digit_count + 1)
                });

            if digit_count == 100 {
                Some(sum)
            } else {
                None
            }
        })
        .sum();

    println!("{}", total);
}

fn square_digits(num: i32) -> impl Iterator<Item = i32> {
    let mut num_digits = number_pairs(num);
    let mut p = BigInt::zero();
    let mut remainder = BigInt::zero();

    std::iter::from_fn(move || {
        let mut c: BigInt = &remainder * 100;
        if let Some(digits) = num_digits.next() {
            c += digits;
        }

        if c.is_zero() {
            return None;
        }

        let x = (0..)
            .take_while(|&x| (x * (20 * &p + x)) <= c)
            .last()
            .unwrap();

        let y = x * (20 * &p + x);
        remainder = c - y;
        p = &p * 10 + x;

        Some(x)
    })
}

fn number_pairs(num: i32) -> impl Iterator<Item = u8> {
    let mut digits: Vec<u8> = digits::new(num).rev().collect();

    if digits.len() % 2 == 1 {
        digits.push(0);
    }

    std::iter::from_fn(move || {
        let d1 = digits.pop()?;
        let d2 = digits.pop()?;

        Some(d1 * 10 + d2)
    })
}
