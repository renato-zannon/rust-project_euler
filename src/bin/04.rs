/* Problem 4: Largest palindrome product
 *
 * A palindromic number reads the same both ways. The largest palindrome made from the product of
 * two 2-digit numbers is 9009 = 91 × 99.
 *
 * Find the largest palindrome made from the product of two 3-digit numbers. */

#![allow(unstable)]
extern crate shared;

use std::iter::range_inclusive;
use std::iter::Unfold;

use shared::combinations;

fn main() {
    let pairs = combinations::new(range_inclusive(100 as usize, 999).collect());

    let (result, (n1, n2)) = pairs
        .filter(|&(n1, n2): &(usize, usize)| n1 <= n2)
        .filter_map(select_palindromes)
        .max_by(|&(n, _)| n)
        .unwrap();

    println!("{} ({}, {})", result, n1, n2);
}

fn select_palindromes(pair: (usize, usize)) -> Option<(usize, (usize, usize))> {
    let &(n1, n2) = &pair;
    let mult = n1 * n2;

    if is_palindrome(mult) {
        Some((mult, (n1, n2)))
    } else {
        None
    }
}

fn is_palindrome(n: usize) -> bool {
    n == reverse(n)
}

fn reverse(n: usize) -> usize {
    return Unfold::new((n, 0), unfold_reverse).last().unwrap();

    fn unfold_reverse(state_ptr: &mut (usize, usize)) -> Option<usize> {
        let &mut (remaining, reversed) = state_ptr;

        if remaining <= 0 {
            return None;
        }

        let new_remaining = remaining / 10;
        let div_remainder = remaining % 10;

        let new_reversed = reversed * 10 + div_remainder;

        *state_ptr = (new_remaining, new_reversed);

        Some(new_reversed)
    }
}
