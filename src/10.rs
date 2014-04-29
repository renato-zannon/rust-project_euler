/* Problem 10: Summation of primes
 *
 * The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
 *
 * Find the sum of all the primes below two million. */

static PRIME_MAX: uint = 2_000_000;

fn main() {
  let result = primes::new()
    .take_while(|&prime| prime < PRIME_MAX)
    .fold(0u64, |acc, num| acc + (num as u64));

  println!("{}", result);
}

pub mod primes {
  use std::iter::count;
  use std::iter::range_step_inclusive;

  pub struct Primes {
    last: uint,
  }

  pub fn new() -> Primes {
    Primes { last: 1 }
  }

  impl Iterator<uint> for Primes {
    fn next(&mut self) -> Option<uint> {
      compute_next(self.last).map(|num| { self.last = num; num })
    }
  }

  fn compute_next(last: uint) -> Option<uint> {
    let initial = last + 1;

    if initial == 2 || initial == 3 { return Some(initial); }

    count(initial, 1)
      .filter(|num| num % 2 > 0 && num % 3 > 0)
      .filter(|num| (num + 1) % 6 == 0 || (num - 1) % 6 == 0)
      .find(is_prime)
  }

  fn is_prime(&num: &uint) -> bool {
    let ceil = (num as f64).sqrt().ceil() as uint;

    // Being divisble by 2 or 3 was already discarded
    range_step_inclusive(5, ceil, 6).all(|candidate1| {
      let candidate2 = candidate1 + 2;

      num % candidate1 > 0 && num % candidate2 > 0
    })
  }
}
