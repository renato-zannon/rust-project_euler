// Based on the ruby implementation:
// https://github.com/ruby/ruby/blob/1aa54bebaf274bc08e72f9ad3854c7ad592c344a/lib/prime.rb#L423

use std::iter::RandomAccessIterator;

static WHEEL: &'static [uint] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101];

static MAX_SEGMENT_SIZE: uint = 1_000_000u;

pub struct Sieve {
  last_prime_index: Option<uint>,
  max_checked: uint,
  primes: Vec<uint>,
}

pub fn new() -> Sieve {
  let primes = Vec::from_slice(WHEEL);

  Sieve {
    last_prime_index: None,
    max_checked: *primes.last().unwrap(),
    primes: primes,
  }
}

impl Iterator<uint> for Sieve {
  fn next(&mut self) -> Option<uint> {
    let index = match self.last_prime_index {
      Some(last_index) => last_index + 1,
      None             => 0
    };

    loop {
      match self.primes.as_slice().get(index) {
        Some(&prime) => {
          self.last_prime_index = Some(index);
          return Some(prime);
        },

        None => self.compute_primes()
      }
    }
  }
}

impl RandomAccessIterator<uint> for Sieve {
  fn indexable(&self) -> uint {
    use std::uint;

    uint::MAX
  }

  fn idx(&mut self, index: uint) -> Option<uint> {
    loop {
      match self.primes.as_slice().get(index) {
        Some(&prime) => {
          return Some(prime);
        },

        None => self.compute_primes()
      }
    }
  }
}

struct Segment {
  min: uint,
  max: uint,
  len: uint,
  values: Vec<Option<uint>>,
}

impl Sieve {
  pub fn is_prime(&mut self, number: uint) -> bool {
    while *self.primes.last().unwrap() < number {
      self.compute_primes();
    }

    match self.primes.as_slice().bsearch_elem(&number) {
      Some(_) => true,
      None    => false,
    }
  }

  fn compute_primes(&mut self) {
    use std::iter::range_step;

    let mut segment = self.next_segment();

    for &prime in self.sieving_primes(segment.max).iter() {
      let first_composite = (prime - (segment.min % prime)) % prime;

      for composite_index in range_step(first_composite, segment.len, prime) {
        *segment.values.get_mut(composite_index) = None;
      }
    }

    self.max_checked = segment.max - 1;

    for maybe_num in segment.values.move_iter() {
      match maybe_num {
        Some(prime) => self.primes.push(prime),
        None        => (),
      }
    }
  }

  fn sieving_primes<'a>(&'a self, max: uint) -> &'a [uint] {
    let root = (max as f64).sqrt().floor() as uint;

    let last = self.primes.iter().position(|&prime| {
      prime > root
    }).unwrap();

    self.primes.slice_to(last)
  }

  fn next_segment(&self) -> Segment {
    use std::cmp;

    let max_cached_prime = *self.primes.last().unwrap();

    let min = self.max_checked + 1;
    let max = cmp::min(max_cached_prime * 2, min + MAX_SEGMENT_SIZE);

    let len = max - min;

    let values = Vec::from_fn(len, |index| Some(min + index));

    Segment {
      min: min,
      max: max,
      len: len,
      values: values
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
