/* Problem 36: Double-base palindromes
 *
 * The decimal number, 585 = 10010010012 (binary), is palindromic in both bases.
 *
 * Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.
 *
 * (Please note that the palindromic number, in either base, may not include leading zeros.) */

extern crate shared;
use std::fmt::Write;

trait NumberFormatter {
    fn format(number: u32, buffer: &mut String);
}

struct Binary;
struct Decimal;

impl NumberFormatter for Binary {
    fn format(number: u32, buffer: &mut String) {
        write!(buffer, "{:b}", number).unwrap();
    }
}

impl NumberFormatter for Decimal {
    fn format(number: u32, buffer: &mut String) {
        write!(buffer, "{}", number).unwrap();
    }
}

fn main() {
    let mut b1 = String::with_capacity(50);
    let mut b2 = String::with_capacity(50);

    let result: u32 = (1..1_000_000)
        .filter(|number| is_palindrome::<Decimal>(*number, &mut b1))
        .filter(|number| is_palindrome::<Binary>(*number, &mut b2))
        .sum();

    println!("{}", result);
}

fn is_palindrome<T: NumberFormatter>(number: u32, buffer: &mut String) -> bool {
    buffer.clear();
    T::format(number, buffer);

    buffer
        .chars()
        .zip(buffer.chars().rev())
        .all(|(from_start, from_end)| from_start == from_end)
}
