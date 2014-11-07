/* Problem 23: Non-abundant sums
 *
 * A perfect number is a number for which the sum of its proper divisors is exactly equal to the
 * number. For example, the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28, which
 * means that 28 is a perfect number.
 *
 * A number n is called deficient if the sum of its proper divisors is less than n and it is called
 * abundant if this sum exceeds n.
 *
 * As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can be
 * written as the sum of two abundant numbers is 24. By mathematical analysis, it can be shown that
 * all integers greater than 28123 can be written as the sum of two abundant numbers. However, this
 * upper limit cannot be reduced any further by analysis even though it is known that the greatest
 * number that cannot be expressed as the sum of two abundant numbers is less than this limit.
 *
 * Find the sum of all the positive integers which cannot be written as the sum of two abundant
 * numbers.*/

extern crate num;

use std::iter::{range_inclusive, range_step_inclusive};
use std::iter::AdditiveIterator;
use num::Integer;
use std::collections::HashSet;

const MAX_NON_ABUNDANT: uint = 28123;

fn main() {
    println!("{}", non_abundant_number_sums());
}

fn non_abundant_number_sums() -> uint {
    let sums = abundant_numbers_sum(abundant_numbers_below(MAX_NON_ABUNDANT));

    range_inclusive(1, MAX_NON_ABUNDANT).filter(|num| {
        ! sums.contains(num)
    }).sum()
}

fn abundant_numbers_sum(abundant_numbers: Vec<uint>) -> HashSet<uint> {
    let mut result = HashSet::new();

    for (index, &n1) in abundant_numbers.init().iter().enumerate() {
        for &n2 in abundant_numbers.slice_from(index).iter() {
            let sum = n1 + n2;

            if sum <= MAX_NON_ABUNDANT {
                result.insert(sum);
            } else {
                break
            }
        }
    }

    result
}

fn abundant_numbers_below(ceil: uint) -> Vec<uint> {
    let mut result = vec!();

    /* 12 is given as the first abundant by the problem  statement */
    result.push(12);

    for current in range_inclusive(13, ceil) {
        if proper_divisor_sum(current) > current {
            result.push(current);
        }
    }

    result
}

fn proper_divisor_sum(number: uint) -> uint {
    let mut result = 1;

    let last_candidate = {
        let sqrt = (number as f64).sqrt().floor() as uint;

        if sqrt * sqrt == number {
            result += sqrt;
            sqrt - 1
        } else {
            sqrt
        }
    };

    let (first_candidate, step) =
        if number.is_odd() {
            (3, 2)
        } else {
            (2, 1)
        };

    for candidate in range_step_inclusive(first_candidate, last_candidate, step) {
        if number % candidate == 0 {
            result += candidate + (number / candidate);
        }
    }

    result
}
