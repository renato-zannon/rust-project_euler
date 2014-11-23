/* Problem 57: Square root convergents
 *
 * It is possible to show that the square root of two can be expressed as an infinite continued
 * fraction.
 * √2 = 1 + 1/(2 + 1/(2 + 1/(2 + ... ))) = 1.414213...
 *
 * By expanding this for the first four iterations, we get:
 * 1 + 1/2 = 3/2 = 1.5
 * 1 + 1/(2 + 1/2) = 7/5 = 1.4
 * 1 + 1/(2 + 1/(2 + 1/2)) = 17/12 = 1.41666...
 * 1 + 1/(2 + 1/(2 + 1/(2 + 1/2))) = 41/29 = 1.41379...
 *
 * The next three expansions are 99/70, 239/169, and 577/408, but the eighth expansion, 1393/985, is
 * the first example where the number of digits in the numerator exceeds the number of digits in the
 * denominator.  In the first one-thousand expansions, how many fractions contain a numerator with more
 * digits than denominator? */

const ITERATION_COUNT: uint = 1000;

// Starting with x0 = a0/b0 = 3/2, xn+1 = an + 2bn / bn + an
fn main() {
    let mut a = vec![3u8];
    let mut b = vec![2u8];

    let mut count = 0u32;
    for _ in range(0, ITERATION_COUNT) {
        advance(&mut a, &mut b);

        if a.len() > b.len() {
            count += 1;
        }
    }

    println!("{}", count);
}

fn advance(a: &mut Vec<u8>, b: &mut Vec<u8>) {
    use std::{mem, cmp};

    // Store "old_a + old_b" = new_b into a.
    sum(b.as_slice(), a);

    // Sum a (which is "old_a + old_b") with b (which is "old_b") and store on b. Now b contains
    // "old_a + 2old_b" = new_a
    sum(a.as_slice(), b);

    // a contains new_b and b contains new_a. Swap them.
    mem::swap(a, b);
    return;

    fn sum(source: &[u8], destination: &mut Vec<u8>) {
        let mut carry: u8 = 0;
        let digit_count = cmp::max(source.len(), destination.len());

        for index in range(0, digit_count) {
            let result = {
                let source_digit = *source.get(index).unwrap_or(&0);
                let dest_digit   = *destination.as_slice().get(index).unwrap_or(&0);

                source_digit + dest_digit + carry
            };

            let result_digit = result % 10;
            let result_carry = result / 10;

            if index >= destination.len() {
                destination.push(result_digit);
            } else {
                destination[index] = result_digit;
            }

            carry = result_carry;
        }

        while carry > 0 {
            let digit = carry % 10;
            carry = carry / 10;

            destination.push(digit);
        }
    }
}
