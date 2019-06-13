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

use num::Integer;
use std::iter;

const MAX_NON_ABUNDANT: u32 = 28123;

fn main() {
    println!("{}", non_abundant_number_sums());
}

fn non_abundant_number_sums() -> u32 {
    let numbers = abundant_numbers_sum(abundant_numbers_below(MAX_NON_ABUNDANT));

    numbers
        .into_iter()
        .enumerate()
        .filter_map(|(index, &result)| match result {
            IntegerResult::CanBeWrittenAsSum => None,
            IntegerResult::CannotBeWrittenAsSum => Some(index as u32),
        })
        .fold(0, |sum, number| sum + number)
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum IntegerResult {
    CanBeWrittenAsSum,
    CannotBeWrittenAsSum,
}

fn abundant_numbers_sum(
    abundant_numbers: Vec<u32>,
) -> [IntegerResult; 1 + MAX_NON_ABUNDANT as usize] {
    let mut result = [IntegerResult::CannotBeWrittenAsSum; 1 + MAX_NON_ABUNDANT as usize];

    for (index, &n1) in abundant_numbers[..abundant_numbers.len() - 1]
        .iter()
        .enumerate()
    {
        for &n2 in abundant_numbers[index..].iter() {
            let sum = n1 + n2;

            if sum <= MAX_NON_ABUNDANT {
                result[sum as usize] = IntegerResult::CanBeWrittenAsSum;
            } else {
                break;
            }
        }
    }

    result
}

fn abundant_numbers_below(ceil: u32) -> Vec<u32> {
    let abundants = (13..ceil + 1).filter(|&number| proper_divisor_sum(number) > number);

    /* 12 is given as the first abundant by the problem statement */
    iter::once(12).chain(abundants).collect()
}

fn proper_divisor_sum(number: u32) -> u32 {
    let mut result = 1;

    let last_candidate = {
        let sqrt = (number as f64).sqrt().floor() as u32;

        if sqrt * sqrt == number {
            result += sqrt;
            sqrt - 1
        } else {
            sqrt
        }
    };

    let (first_candidate, step) = if number.is_odd() { (3, 2) } else { (2, 1) };

    for candidate in (first_candidate..last_candidate + 1).step_by(step) {
        if number % candidate == 0 {
            result += candidate + (number / candidate);
        }
    }

    result
}
