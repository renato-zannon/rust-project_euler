/* Problem 43: Sub-string divisibility
 *
 * The number, 1406357289, is a 0 to 9 pandigital number because it is made up of each of the digits
 * 0 to 9 in some order, but it also has a rather interesting sub-string divisibility property.
 *
 * Let d1 be the 1st digit, d2 be the 2nd digit, and so on. In this way, we note the following:
 *
 *     d2d3d4=406 is divisible by 2
 *     d3d4d5=063 is divisible by 3
 *     d4d5d6=635 is divisible by 5
 *     d5d6d7=357 is divisible by 7
 *     d6d7d8=572 is divisible by 11
 *     d7d8d9=728 is divisible by 13
 *     d8d9d10=289 is divisible by 17
 *
 * Find the sum of all 0 to 9 pandigital numbers with this property. */

#![feature(slicing_syntax)]

extern crate shared;

use std::iter::{AdditiveIterator, range_step};
use shared::digits;

const DIVISORS: &'static [u32] = &[2, 3, 5, 7, 11, 13, 17];

fn main() {
    // Start with a collection of all 3-digit numbers divisible by 17 that don't
    // that don't have repeated digits
    let bases = range_step(102, 1000, 17).filter_map(|multiple| {
        let mut found_digits = [false; 10];
        let num_digits = digits::new(multiple).collect::<Vec<u32>>();

        for &digit in num_digits.iter() {
            let index = digit as usize;

            if found_digits[index] {
                return None;
            } else {
                found_digits[index] = true;
            }
        }

        Some(num_digits)
    }).collect::<Vec<_>>();

    let divisible_by_all = DIVISORS.init().iter().rev().fold(bases, |numbers, &divisor| {
        let mut next_digits: Vec<Vec<u32>> = Vec::new();

        for digits in numbers.into_iter() {
            let next = plus_one_digit(digits).filter(|more_digits| {
                to_number(more_digits.slice_to(3)) % divisor == 0
            });

            next_digits.extend(next);
        }

        next_digits
    });

    let result = divisible_by_all.into_iter()
        .map(|digits| to_number(&digits[]))
        .sum();

    println!("{}", result);
}

fn to_number(digits: &[u32]) -> u32 {
    digits.iter().fold(0, |acc, &digit| {
        acc * 10 + digit
    })
}

fn plus_one_digit(base: Vec<u32>) -> PlusOneDigit {
    let mut used_digits = [false; 10];

    for &digit in base.iter() {
        let index = digit as usize;
        used_digits[index] = true;
    }

    PlusOneDigit {
        used_digits: used_digits,
        base: base,
    }
}

struct PlusOneDigit {
    used_digits: [bool; 10],
    base: Vec<u32>,
}

impl Iterator for PlusOneDigit {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Vec<u32>> {
        self.get_next_digit().map(|next_digit| {
            let mut combination = Vec::with_capacity(self.base.capacity() + 1);

            combination.push(next_digit);
            combination.push_all(&self.base[]);

            let index = next_digit as usize;
            self.used_digits[index] = true;

            combination
        })
    }
}

impl PlusOneDigit {
    fn get_next_digit(&self) -> Option<u32> {
        for (index, &was_used) in self.used_digits.iter().enumerate() {
            if !was_used {
                let digit = index as u32;
                return Some(digit);
            }
        }

        None
    }
}
