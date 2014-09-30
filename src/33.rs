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
use std::collections::hashmap::HashSet;
use num::integer::gcd;

fn main() {
    let mut result = (1, 1);

    for numerator in range(10u, 100u) {
        for denominator in range(numerator + 1, 100u) {
            match cancel_fraction(numerator, denominator) {
                NonCancelable => continue,
                Cancelable(num, den) => {
                    result = (result.val0() * num, result.val1() * den);
                }
            }
        }
    }

    println!("{}", simplify(result));
}

enum CancelResult {
    NonCancelable,
    Cancelable(uint, uint),
}

fn cancel_fraction(numerator: uint, denominator: uint) -> CancelResult {
    let num_digits: Vec<uint> = digits::new(numerator).collect();
    let den_digits: Vec<uint> = digits::new(denominator).collect();

    let shared_digits: HashSet<uint> = num_digits.iter()
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

fn build_from_digits(digits: Vec<uint>, except: &HashSet<uint>) -> uint {
    use std::num::pow;

    let mut used_shared = HashSet::with_capacity(except.len());

    let used_digits = digits.into_iter().filter(|digit| {
        if except.contains(digit) && !used_shared.contains(digit) {
            used_shared.insert(*digit);
            false
        } else {
            true
        }
    });

    used_digits.enumerate().fold(0, |sum, (power, digit)| {
        sum + digit * pow(10u, power)
    })
}

fn simplify((numerator, denominator): (uint, uint)) -> (uint, uint) {
    let divisor = gcd(numerator, denominator);

    (numerator / divisor, denominator / divisor)
}
