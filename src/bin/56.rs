/* Problem 56: Powerful digit sum
 *
 * A googol (10¹⁰⁰) is a massive number: one followed by one-hundred zeros; 100¹⁰⁰ is almost
 * unimaginably large: one followed by two-hundred zeros. Despite their size, the sum of the digits
 * in each number is only 1.
 *
 * Considering natural numbers of the form, ab, where a, b < 100, what is the maximum digital sum?
 * */

extern crate shared;
use shared::digits;
use std::mem;

const MAX_DIGITS: uint = 200;

const MAX_BASE: uint = 100;
const MAX_EXP:  uint = 100;

fn main() {
    let mut max_sum = 0;

    for base in range(2, MAX_BASE) {
        let mut buffer      = Vec::with_capacity(MAX_DIGITS);
        let mut prev_buffer = Vec::with_capacity(MAX_DIGITS);

        prev_buffer.extend(digits::new(base).rev());

        for _ in range(2, MAX_EXP) {
            buffer.clear();

            let mut carry: uint = 0;
            let mut sum:   uint = 0;

            for &old_digit in prev_buffer.iter() {
                let result = multiply_digit(old_digit, base, carry);

                carry = result.carry;
                sum  += result.digit;
                buffer.push(result.digit);
            }

            if carry > 0 {
                for carry_digit in digits::new(carry).rev() {
                    buffer.push(carry_digit);
                    sum += carry_digit;
                }
            }

            if sum > max_sum {
                max_sum = sum;
            }

            mem::swap(&mut prev_buffer, &mut buffer);
        }
    }

    println!("{}", max_sum);
}

struct MultiplicationResult {
    digit: uint,
    carry: uint,
}

fn multiply_digit(digit: uint, multiplier: uint, carry: uint) -> MultiplicationResult {
    let value = digit * multiplier + carry;

    let new_digit = value % 10;
    let new_carry = value / 10;

    MultiplicationResult { digit: new_digit, carry: new_carry }
}
