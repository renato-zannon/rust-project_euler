/* Problem 33: Digit canceling fractions
 *
 * The fraction 49/98 is a curious fraction, as an inexperienced mathematician in attempting to
 * simplify it may incorrectly believe that 49/98 = 4/8, which is correct, is obtained by cancelling
 * the 9s.
 *
 * We shall consider fractions like, 30/50 = 3/5, to be trivial examples.
 *
 * There are exactly four non-trivial examples of this type of fraction, less than one in value, and
 * containing two digits in the numerator and denominator.
 *
 * If the product of these four fractions is given in its lowest common terms, find the value of the
 * denominator. */

extern crate shared;
extern crate num;

use shared::digits;
use std::collections::HashSet;
use num::integer::gcd;
use self::CancelResult::{Cancelable, NonCancelable};

fn main() {
    let mut result = (1, 1);

    for numerator in 10u32..100 {
        for denominator in (numerator + 1)..100 {
            match cancel_fraction(numerator, denominator) {
                NonCancelable => continue,
                Cancelable(num, den) => {
                    result = (result.0 * num, result.1 * den);
                }
            }
        }
    }

    println!("{:?}", simplify(result));
}

enum CancelResult {
    NonCancelable,
    Cancelable(u32, u32),
}

fn cancel_fraction(numerator: u32, denominator: u32) -> CancelResult {
    let num_digits: Vec<u32> = digits::new(numerator).collect();
    let den_digits: Vec<u32> = digits::new(denominator).collect();

    let shared_digits: HashSet<u32> = num_digits.iter()
        .map(|&digit| digit)
        .filter(|digit| digit != &0 && den_digits.contains(digit))
        .collect();

    if shared_digits.len() == 0 {
        return NonCancelable;
    }

    let cancelled_numerator   = build_from_digits(num_digits, &shared_digits);
    let cancelled_denominator = build_from_digits(den_digits, &shared_digits);

    let matching_fractions = !(cancelled_denominator == 0 || cancelled_numerator == 0) &&
        numerator * cancelled_denominator == denominator * cancelled_numerator;

    if matching_fractions {
        Cancelable(cancelled_numerator, cancelled_denominator)
    } else {
        NonCancelable
    }
}

fn build_from_digits(digits: Vec<u32>, except: &HashSet<u32>) -> u32 {
    let mut used_shared = HashSet::with_capacity(except.len());

    let used_digits = digits.into_iter().filter(|digit| {
        if except.contains(digit) && !used_shared.contains(digit) {
            used_shared.insert(*digit);
            false
        } else {
            true
        }
    });

    used_digits.enumerate().fold(0, |sum, (index, digit)| {
        let power = index as u32;
        sum + digit * 10u32.pow(power)
    })
}

fn simplify((numerator, denominator): (u32, u32)) -> (u32, u32) {
    let divisor = gcd(numerator, denominator);

    (numerator / divisor, denominator / divisor)
}
