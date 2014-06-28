/* Problem 36: Double-base palindromes
 * 
 * The decimal number, 585 = 10010010012 (binary), is palindromic in both bases.
 * 
 * Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.
 * 
 * (Please note that the palindromic number, in either base, may not include leading zeros.) */

use std::iter::AdditiveIterator;
use std::num::ToStrRadix;

fn main() {
  let result = range(1u, 1_000_000).filter(|number| {
    is_palindrome(number.to_str()) && is_palindrome(number.to_str_radix(2))
  }).sum();

  println!("{}", result);
}

fn is_palindrome(string: String) -> bool {
  let slice = string.as_slice();

  slice.chars().zip(slice.chars().rev()).all(|(from_start, from_end)| {
    from_start == from_end
  })
}
