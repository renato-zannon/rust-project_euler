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

extern crate num;

use num::bigint::BigInt;

fn main() {
  let fibonacci = {
    use std::num::one;
    use std::iter::Unfold;

    // Rust's BigInt are notoriously inconvenient here: We can't use a literal '1',
    // because it would be interpreted as a 'native' number. The one() function leverages trait
    // dispatch and returns a BigInt '1'
    let core_fib = Unfold::new((one(), one()), fib_iteration);
    let initial  = vec!(one(), one()).into_iter();

    initial.chain(core_fib)
  };

  let (index, _) = fibonacci.scan(1, |prev, num| {
    if *prev >= 1000 {
      None
    } else {
      let result = number_of_digits(&num, *prev);
      *prev = result;
      Some(num)
    }
  }).enumerate().last().unwrap();

  println!("{}", index + 1);
}

fn fib_iteration(state: &mut (BigInt, BigInt)) -> Option<BigInt> {
  use std::mem::swap;

  let &(ref mut pprev, ref mut prev) = state;
  let result = *pprev + *prev;

  swap(pprev, prev);
  *prev = result.clone();

  Some(result)
}

fn number_of_digits(num: &BigInt, minimum_digits: uint) -> uint {
  use std::num::zero;

  let bigzero: BigInt = zero();
  let big10:   BigInt = from_str("10").unwrap();

  let mut digits = minimum_digits;

  let mut remaining = {
    let mut buf = String::with_capacity(minimum_digits + 1);
    buf.push('1');
    buf.grow(minimum_digits, '0');

    let minimum = from_str(buf.as_slice()).unwrap();
    num / minimum
  };

  while remaining > bigzero {
    remaining = remaining / big10;
    digits += 1;
  }

  digits
}
