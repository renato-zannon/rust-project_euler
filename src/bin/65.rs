/* Problem 65: Convergents of e
 *
 * The square root of 2 can be written as an infinite continued fraction.
 * √2 = 1 + (1/(2 + 1/(2 + 1/(2 + 1/(2 + 1/(2 + ...)))
 *
 * The infinite continued fraction can be written, √2 = [1;(2)], (2) indicates that 2 repeats ad
 * infinitum. In a similar way, √23 = [4;(1,3,1,8)].
 *
 * It turns out that the sequence of partial values of continued fractions for square roots provide
 * the best rational approximations. Let us consider the convergents for √2.
 *
 * 1 + 1/2 = 3/2
 * 1 + 1/(2 + 1/2) = 7/5
 * 1 + 1/(2 + 1/(2 + 1/2)) = 17/12
 * 1 + 1/(2 + 1/(2 + 1/(2 + 1/2))) = 41/29
 *
 * Hence the sequence of the first ten convergents for √2 are:
 * 1, 3/2, 7/5, 17/12, 41/29, 99/70, 239/169, 577/408, 1393/985, 3363/2378, ...
 *
 * What is most surprising is that the important mathematical constant,
 * e = [2; 1,2,1, 1,4,1, 1,6,1 , ... , 1,2k,1, ...].
 *
 * The first ten terms in the sequence of convergents for e are:
 * 2, 3, 8/3, 11/4, 19/7, 87/32, 106/39, 193/71, 1264/465, 1457/536, ...
 *
 * The sum of digits in the numerator of the 10th convergent is 1+4+5+7=17.
 *
 * Find the sum of digits in the numerator of the 100th convergent of the continued fraction for e */


extern crate num;
extern crate shared;

use shared::digits;
use std::num::Int;
use std::cmp;
use std::iter::repeat;

const INITIAL_TERM: u32 = 2;
const CONVERGENT_INDEX: usize = 99;

fn main() {
    let (terms, last_term) = {
        let mut iter = Term::initial();

        let terms: Vec<_> = [INITIAL_TERM].iter().map(|&n| n)
            .chain(iter.by_ref())
            .take(CONVERGENT_INDEX).collect();

        let last = iter.next().unwrap();

        (terms, last)
    };


    let last_fraction: (Vec<u8>, Vec<u8>) = (
        digits::new(last_term).rev().collect(),
        vec![1]
    );

    let rev_terms = terms.into_iter().rev();

    // t + 1/(n/d) => t + d/n => (nt + d)/n
    let (result, _) = rev_terms.fold(last_fraction, |(last_numerator, last_denominator), term| {
        let new_numerator = calculate_numerator(term, &*last_numerator, &*last_denominator);
        let new_denominator = last_numerator;

        (new_numerator, new_denominator)
    });

    println!("{}", result.into_iter().fold(0u16, |acc, n| acc + (n as u16)));
}

fn calculate_numerator(term: u32, numerator: &[u8], denominator: &[u8]) -> Vec<u8> {
    let numerator_iter   = numerator.iter().map(|&n| n).chain(repeat(0));
    let denominator_iter = denominator.iter().map(|&n| n).chain(repeat(0));

    let sum_len = cmp::max(numerator.len(), denominator.len());
    let mut final_number = Vec::with_capacity(sum_len);

    let mut carry: u32 = 0;

    for (num_digit, denom_digit) in numerator_iter.zip(denominator_iter).take(sum_len) {
        let result: u32 = (num_digit as u32) * term + (denom_digit as u32) + carry;
        carry = result / 10;

        let digit: u8 = (result % 10) as u8;
        final_number.push(digit);
    }

    if carry > 0 {
        final_number.extend(digits::new(carry).rev());
    }

    final_number
}

enum Term {
    LeadingOne(u32),
    Multiple(u32),
    TrailingOne(u32),
}

impl Term {
    fn initial() -> Term {
        Term::LeadingOne(2)
    }

    fn value(&self) -> u32 {
        match *self {
            Term::LeadingOne(_) | Term::TrailingOne(_) => 1,
            Term::Multiple(v)                          => v,
        }
    }
}

impl Iterator for Term {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let new_term = match *self {
            Term::LeadingOne(v)  => Some(Term::Multiple(v)),
            Term::Multiple(v)    => Some(Term::TrailingOne(v)),
            Term::TrailingOne(v) => v.checked_add(2).map(|n| Term::LeadingOne(n))
        };

        new_term.map(|term| {
            let value = self.value();
            *self = term;
            value
        })
    }
}
