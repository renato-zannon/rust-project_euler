/* Problem 36: Double-base palindromes
 *
 * The decimal number, 585 = 10010010012 (binary), is palindromic in both bases.
 *
 * Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.
 *
 * (Please note that the palindromic number, in either base, may not include leading zeros.) */

#![feature(fmt_radix)]
extern crate shared;
use std::fmt::{self, Write};

fn main() {
    let mut buffer = String::with_capacity(50);

    let result = (1..1_000_000).filter(|number| {
        is_palindrome(*number, 10, &mut buffer) && is_palindrome(*number, 2, &mut buffer)
    }).fold(0, |acc, num| acc + num);

    println!("{}", result);
}

fn is_palindrome(number: u32, base: u8, buffer: &mut String) -> bool {
    buffer.clear();
    write!(buffer, "{}", fmt::radix(number, base)).unwrap();

    buffer.chars().zip(buffer.chars().rev()).all(|(from_start, from_end)| {
        from_start == from_end
    })
}
