extern crate collections;
use collections::treemap::TreeMap;
use std::iter::count;

pub struct Sieve {
  last_test: uint,
  checked: TreeMap<uint, uint>
}

pub fn new() -> Sieve {
  let map = TreeMap::new();

  Sieve {
    last_test: 1,
    checked: map,
  }
}

impl Iterator<uint> for Sieve {
  fn next(&mut self) -> Option<uint> {
    count(self.last_test + 1, 1).find(|&number| {
      if self.is_prime(number) {
        self.checked.insert(number, number);
        self.last_test = number;
        true
      } else {
        false
      }
    })
  }
}

impl Sieve {
  fn is_composite(&mut self, number: uint) -> bool {
    self.checked.mut_iter().any(|(&prime, checked_num)| {
      while number > *checked_num {
        *checked_num += prime;
      }

      *checked_num == number
    })
  }

  #[inline]
  fn is_prime(&mut self, number: uint) -> bool {
    ! self.is_composite(number)
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
