/* Problem 36: Double-base palindromes
 *
 * The decimal number, 585 = 10010010012 (binary), is palindromic in both bases.
 *
 * Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.
 *
 * (Please note that the palindromic number, in either base, may not include leading zeros.) */

#![feature(core, io)]
use std::iter::AdditiveIterator;
use std::fmt;

fn main() {
    let result = (1us..1_000_000).filter(|number| {
        is_palindrome(*number, 10) && is_palindrome(*number, 2)
    }).sum();

    println!("{}", result);
}

fn is_palindrome(number: usize, base: u8) -> bool {
    use std::old_io::BufWriter;
    use std::str;

    let mut buffer = [0u8; 50];

    let slice = {
        let mut writer = BufWriter::new(&mut buffer);

        (write!(&mut writer, "{}", fmt::radix(number, base))).and_then(|_| {
            return writer.tell()
        })
    }.ok().and_then(|size| {
        str::from_utf8(&buffer[..size as usize]).ok()
    }).unwrap();

    slice.chars().zip(slice.chars().rev()).all(|(from_start, from_end)| {
        from_start == from_end
    })
}
