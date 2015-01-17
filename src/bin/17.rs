/* Problem 17: Number letter counts
 *
 * If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 +
 * 5 + 4 + 4 = 19 letters used in total.
 *
 * If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many
 * letters would be used?
 *
 * NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23
 * letters and 115 (one hundred and fifteen) contains 20 letters. The use of "and" when writing out
 * numbers is in compliance with British usage. */

use std::iter::range_inclusive;

fn main() {
    let result = range_inclusive(1us, 1000).fold(0us, |sum, number| {
        sum + letter_count(number)
    });

    println!("{}", result);
}

fn letter_count(num: usize) -> usize {
    match num {
        1  => 3, // "One"
        2  => 3, // "Two"
        3  => 5, // "Three"
        4  => 4, // "Four"
        5  => 4, // "Five"
        6  => 3, // "Six"
        7  => 5, // "Seven"
        8  => 5, // "Eight"
        9  => 4, // "Nine"
        10 => 3, // "Ten"
        11 => 6, // "Eleven"
        12 => 6, // "Twelve"
        13 => 8, // "Thirteen"
        14 => 8, // "Fourteen"
        15 => 7, // "Fifteen"
        16 => 7, // "Sixteen"
        17 => 9, // "Seventeen"
        18 => 8, // "Eighteen"
        19 => 8, // "Nineteen"

        20      => 6, // "Twenty"
        21...29 => 6 + letter_count(num % 20), // "Twenty-XX",

        30      => 6, // "Thirty"
        31...39 => 6 + letter_count(num % 30), // "Thirty-XX",

        40      => 5, // "Forty"
        41...49 => 5 + letter_count(num % 40), // "Forty-XX",

        50      => 5, // "Fifty"
        51...59 => 5 + letter_count(num % 50), // "Fifty-XX",

        60      => 5, // "Sixty"
        61...69 => 5 + letter_count(num % 60), // "Sixty-XX",

        70      => 7, // "Seventy"
        71...79 => 7 + letter_count(num % 70), // "Seventy-XX",

        80      => 6, // "Eighty"
        81...89 => 6 + letter_count(num % 80), // "Eighty-XX",

        90      => 6, // "Ninety"
        91...99 => 6 + letter_count(num % 90), // "Ninety-XX",

        1000 => 11, // "One Thousand"

        num if num % 100 == 0 => letter_count(num / 100) + 7, // "XX hundred"

        num if num < 1000 => letter_count(num / 100) + 10 + letter_count(num % 100), // "XX hundred and YY"

        _ => panic!("Number too big")
    }
}
