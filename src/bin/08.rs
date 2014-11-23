/* Problem 8: Largest product in a series
 *
 * Find the greatest product of five consecutive digits in the 1000-digit number. */

use std::iter::MultiplicativeIterator;

const NUMBER: &'static str = include_str!("../../data/08-number.txt");

const DIGIT_COUNT: uint = 5;

fn main() {
    let (string, result) = find_biggest(consecutive_digits::new(NUMBER, DIGIT_COUNT));
    println!("digits: {}\nproduct: {}", string, result);
}

fn find_biggest(slices: consecutive_digits::ConsecutiveDigits) -> (&str, uint) {
    slices.map(|str| (str, multiply(str)))
        .max_by(|&(_, num)| num)
        .unwrap()
}

fn multiply(num: &str) -> uint {
    num.chars().map(|chr| chr.to_digit(10).unwrap()).product()
}

mod consecutive_digits {
    pub struct ConsecutiveDigits<'a> {
        string: &'a str,
        current: uint,
        length: uint,
    }

    impl<'a> Iterator<&'a str> for ConsecutiveDigits<'a> {
        fn next(&mut self) -> Option<&'a str> {
            let start = self.current;
            let end   = start + self.length;

            if end > self.string.len() - 1 {
                None
            } else {
                self.current = start + 1;
                Some(self.string.slice(start, end))
            }
        }
    }

    pub fn new(string: &str, length: uint) -> ConsecutiveDigits {
        ConsecutiveDigits {
            string: string,
            length: length,
            current: 0,
        }
    }
}
