/* Problem 25: 1000-digit Fibonacci number
 * The Fibonacci sequence is defined by the recurrence relation:
 *
 * Fn = Fn1 + Fn2, where F1 = 1 and F2 = 1.
 * Hence the first 12 terms will be:
 *
 * F1 = 1
 * F2 = 1
 * F3 = 2
 * F4 = 3
 * F5 = 5
 * F6 = 8
 * F7 = 13
 * F8 = 21
 * F9 = 34
 * F10 = 55
 * F11 = 89
 * F12 = 144
 * The 12th term, F12, is the first term to contain three digits.
 *
 * What is the first term in the Fibonacci sequence to contain 1000 digits? */

use num::bigint::BigInt;
use std::iter::repeat;

fn main() {
    let fibonacci = {
        use num::one;

        // Rust's BigInt are notoriously inconvenient here: We can't use a literal '1',
        // because it would be interpreted as a 'native' number. The one() function leverages trait
        // dispatch and returns a BigInt '1'
        let core_fib = Fibonacci {
            prev: one(),
            pprev: one(),
        };
        repeat(one()).take(2).chain(core_fib)
    };

    let (index, _) = fibonacci
        .scan(1, |prev, num| {
            if *prev >= 1000 {
                None
            } else {
                let result = number_of_digits(&num, *prev);
                *prev = result;
                Some(num)
            }
        })
        .enumerate()
        .last()
        .unwrap();

    println!("{}", index + 1);
}

struct Fibonacci {
    pprev: BigInt,
    prev: BigInt,
}

impl Iterator for Fibonacci {
    type Item = BigInt;

    fn next(&mut self) -> Option<BigInt> {
        use std::mem::replace;
        let result = (&self.pprev) + (&self.prev);

        self.pprev = replace(&mut self.prev, result.clone());
        Some(result)
    }
}

fn number_of_digits(num: &BigInt, minimum_digits: usize) -> usize {
    use num::zero;

    let bigzero: BigInt = zero();
    let big10: BigInt = "10".parse().unwrap();

    let mut digits = minimum_digits;

    let mut remaining = {
        let mut buf = String::with_capacity(minimum_digits + 1);

        buf.push('1');
        buf.extend(repeat('0').take(minimum_digits));

        let minimum: BigInt = buf.parse().unwrap();
        num / minimum
    };

    while remaining > bigzero {
        remaining = remaining / &big10;
        digits += 1;
    }

    digits
}
