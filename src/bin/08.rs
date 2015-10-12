/* Problem 8: Largest product in a series
 *
 * Find the greatest product of five consecutive digits in the 1000-digit number. */

const NUMBER: &'static str = include_str!("../../data/08-number.txt");
const DIGIT_COUNT: usize = 5;

fn main() {
    let result = find_biggest(consecutive_digits::new(NUMBER, DIGIT_COUNT));
    println!("{}", result);
}

fn find_biggest(slices: consecutive_digits::ConsecutiveDigits) -> u32 {
    slices.map(multiply).max().unwrap()
}

fn multiply(num: &str) -> u32 {
    num.chars().map(|chr| chr.to_digit(10).unwrap()).fold(1, |factor, product| factor * product)
}

mod consecutive_digits {
    pub struct ConsecutiveDigits<'a> {
        string: &'a str,
        current: usize,
        length: usize,
    }

    impl<'a> Iterator for ConsecutiveDigits<'a> {
        type Item = &'a str;

        fn next(&mut self) -> Option<&'a str> {
            let start = self.current;
            let end   = start + self.length;

            if end > self.string.len() - 1 {
                None
            } else {
                self.current = start + 1;
                Some(&self.string[start..end])
            }
        }
    }

    pub fn new(string: &str, length: usize) -> ConsecutiveDigits {
        ConsecutiveDigits {
            string: string,
            length: length,
            current: 0,
        }
    }
}
