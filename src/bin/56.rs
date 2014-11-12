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

const MAX_BASE: u8 = 100;
const MAX_EXP:  u8 = 100;

fn main() {
    let mut max_sum = 0;

    for base in range(2, MAX_BASE) {
        let mut buffer      = Vec::with_capacity(MAX_DIGITS);
        let mut prev_buffer = Vec::with_capacity(MAX_DIGITS);

        prev_buffer.extend(digits::new(base).rev());

        for _ in range(2, MAX_EXP) {
            buffer.clear();

            let mut carry: u16 = 0;
            let mut sum:   u16 = 0;

            for &old_digit in prev_buffer.iter() {
                let result = multiply_digit(old_digit, base, carry);

                carry = result.carry;
                sum  += result.digit as u16;
                buffer.push(result.digit);
            }

            if carry > 0 {
                for carry_digit in digits::new::<u16, u16>(carry).rev() {
                    buffer.push(carry_digit as u8);
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
    digit: u8,
    carry: u16,
}

fn multiply_digit(digit: u8, multiplier: u8, carry: u16) -> MultiplicationResult {
    let value = (digit as u16) * (multiplier as u16) + carry;

    let new_digit = (value % 10) as u8;
    let new_carry = value / 10;

    MultiplicationResult { digit: new_digit, carry: new_carry }
}
