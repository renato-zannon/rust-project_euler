/* Problem 34: Digit factorials
 *
 * 145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.
 *
 * Find the sum of all numbers which are equal to the sum of the factorial of their digits.
 *
 * Note: as 1! = 1 and 2! = 2 are not sums they are not included. */

extern crate shared;
use shared::digits;

fn main() {
    let max_number = {
        let max_single_digit_factorial = factorial(9);

        let max_digits = (1..)
            .find(|&digits| max_single_digit_factorial * digits < max_number_with_digits(digits))
            .unwrap();

        max_single_digit_factorial * max_digits
    };

    let result = (3u32..max_number)
        .filter(|&number| number_eqls_fact_sum(number))
        .fold(0, |acc, num| acc + num);

    println!("{}", result);
}

fn number_eqls_fact_sum(number: u32) -> bool {
    let mut fact_sum = 0;

    for digit in digits::new(number) {
        fact_sum += factorial(digit);

        if fact_sum > number {
            return false;
        }
    }

    fact_sum == number
}

fn factorial(n: u32) -> u32 {
    (1..n + 1).fold(1, |num, result| num * result)
}

fn max_number_with_digits(digit_count: u32) -> u32 {
    10u32.pow(digit_count) - 1
}
