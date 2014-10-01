/* Problem 4: Largest palindrome product
 *
 * A palindromic number reads the same both ways. The largest palindrome made from the product of
 * two 2-digit numbers is 9009 = 91 × 99.
 *
 * Find the largest palindrome made from the product of two 3-digit numbers. */

extern crate shared;

use std::iter::range_inclusive;
use std::iter::Unfold;
use std::num::div_rem;

use shared::combinations;

fn main() {
    let pairs = combinations::new(range_inclusive(100 as uint, 999).collect());

    let (result, (n1, n2)) = pairs
        .filter(|&(n1, n2): &(uint, uint)| n1 <= n2)
        .filter_map(select_palindromes)
        .max_by(|&(n, _)| n)
        .unwrap();

    println!("{} ({}, {})", result, n1, n2);
}

fn select_palindromes(pair: (uint, uint)) -> Option<(uint, (uint, uint))> {
    let &(n1, n2) = &pair;
    let mult = n1 * n2;

    if is_palindrome(mult) {
        Some((mult, (n1, n2)))
    } else {
        None
    }
}

fn is_palindrome(n: uint) -> bool {
    n == reverse(n)
}

fn reverse(n: uint) -> uint {
    return Unfold::new((n, 0), unfold_reverse).last().unwrap();

    fn unfold_reverse(state_ptr: &mut (uint, uint)) -> Option<uint> {
        let &(remaining, reversed) = state_ptr;

        if remaining <= 0 {
            return None;
        }

        let (new_remaining, div_remainder) = div_rem(remaining, 10);
        let new_reversed = reversed * 10 + div_remainder;

        *state_ptr = (new_remaining, new_reversed);

        Some(new_reversed)
    }
}
