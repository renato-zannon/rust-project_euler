extern crate collections;

use self::collections::treemap::TreeMap;
use std::iter::count;

pub struct Sieve {
  last_prime: Option<uint>,
  checked:    TreeMap<uint, uint>
}

pub fn new() -> Sieve {
  Sieve {
    last_prime: None,
    checked: TreeMap::new(),
  }
}

impl Iterator<uint> for Sieve {
  fn next(&mut self) -> Option<uint> {
    let prime = match self.last_prime {
      None    => 2,
      Some(2) => 3,

      Some(prev_prime) => {
        count(prev_prime + 2, 2)
          .find(|&number| self.is_prime(number))
          .unwrap()
      }
    };

    self.checked.insert(prime, prime * prime);
    self.last_prime = Some(prime);

    Some(prime)
  }
}


impl Sieve {
  #[inline]
  fn is_prime(&mut self, number: uint) -> bool {
    ! self.is_composite(number)
  }

  fn is_composite(&mut self, number: uint) -> bool {
    let max_prime_factor = (number as f64).sqrt().ceil() as uint;

    self.checked.mut_iter().any(|(&prime, prev_composite)| {
      if prime > max_prime_factor { return false; }

      match check_composite(prime, *prev_composite, number) {
        Composite => {
          *prev_composite = number + prime;
          true
        },

        NotComposite(closest_composite) => {
          *prev_composite = closest_composite;
          false
        }
      }
    })
  }
}

enum CompositeCheckResult {
  Composite,
  NotComposite(uint),
}

fn check_composite(prime: uint, prev_composite: uint, number: uint) -> CompositeCheckResult {
  let mut closest_composite = prev_composite;

  loop {
    match closest_composite.cmp(&number) {
      Less    => { closest_composite += prime; },
      Equal   => { return Composite; },
      Greater => { return NotComposite(closest_composite); }
    }
  }
}

#[test]
fn test_first_few_primes() {
  let first_primes = [2, 3, 5, 7, 11, 13, 17, 19, 23];
  let mut iter = new();

  for &prime in first_primes.iter() {
    assert_eq!(iter.next(), Some(prime));
  }
}
