// Based on the ruby implementation:
// https://github.com/ruby/ruby/blob/1aa54bebaf274bc08e72f9ad3854c7ad592c344a/lib/prime.rb#L423

use std::iter::RandomAccessIterator;
use std::num::{from_u8, from_f32, from_uint};

const WHEEL: &'static [uint] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101];

const MAX_SEGMENT_SIZE: uint = 10_000u;

pub trait Primeable : Primitive + Ord + FromPrimitive {}

impl<T> Primeable for T
  where T: Primitive + Ord + FromPrimitive {}


#[deriving(Clone)]
pub struct Sieve<T> {
    last_prime_index: Option<uint>,
    max_checked: T,
    primes: Vec<T>,
}

pub fn new<T: Primeable>() -> Sieve<T> {
    let primes: Vec<T> = WHEEL.iter()
        .map(|&num| from_uint(num).unwrap())
        .collect();

    Sieve {
        last_prime_index: None,
        max_checked: *primes.last().unwrap(),
        primes: primes,
    }
}

impl<T: Primeable> Iterator<T> for Sieve<T> {
    fn next(&mut self) -> Option<T> {
        let index = match self.last_prime_index {
            Some(last_index) => last_index + 1,
            None             => 0
        };

        loop {
            match self.primes[].get(index) {
                Some(&prime) => {
                    self.last_prime_index = Some(index);
                    return Some(prime);
                },

                None => self.compute_primes()
            }
        }
    }
}

impl<T: Primeable> RandomAccessIterator<T> for Sieve<T> {
    fn indexable(&self) -> uint {
        use std::uint;

        uint::MAX
    }

    fn idx(&mut self, index: uint) -> Option<T> {
        loop {
            match self.primes[].get(index) {
                Some(&prime) => {
                    return Some(prime);
                },

                None => self.compute_primes()
            }
        }
    }
}

struct Segment<T> {
    min: T,
    max: T,
    len: uint,
    values: Vec<Option<T>>,
}

impl<T: Primeable> Sieve<T> {
    pub fn is_prime(&mut self, number: T) -> bool {
        use std::slice::{Found, NotFound};

        self.compute_until(number);

        match self.primes[].binary_search_elem(&number) {
            Found(_)    => true,
            NotFound(_) => false,
        }
    }

    pub fn compute_until(&mut self, number: T) {
        while *self.primes.last().unwrap() < number {
            self.compute_primes();
        }
    }

    pub fn found_primes(&self) -> &[T] {
        self.primes[]
    }

    fn compute_primes(&mut self) {
        use std::iter::range_step;

        let mut segment = self.next_segment();

        {
            let seg_values = segment.values.as_mut_slice();

            for &prime in self.sieving_primes(segment.max).iter() {
                let first_composite = (prime - (segment.min % prime)) % prime;

                let first_composite = first_composite.to_uint().unwrap();
                let prime = prime.to_uint().unwrap();

                let mut composites = range_step(first_composite, segment.len, prime);

                for composite_index in composites  {
                    seg_values[composite_index] = None;
                }
            }
        }

        self.max_checked = segment.max - from_u8(1).unwrap();

        for maybe_num in segment.values.into_iter() {
            match maybe_num {
                Some(prime) => self.primes.push(prime),
                None        => (),
            }
        }
    }

    fn sieving_primes(&self, max: T) -> &[T] {
        let root = max.to_f32()
            .map(|as_float| as_float.sqrt())
            .and_then(|result| from_f32(result))
            .unwrap();

        let last = self.primes.iter().position(|&prime| {
            prime > root
        }).unwrap();

        self.primes.slice_to(last)
    }

    fn next_segment(&self) -> Segment<T> {
        use std::cmp;

        let max_cached_prime = *self.primes.last().unwrap();

        let min = self.max_checked + from_u8(1).unwrap();
        let max = cmp::min(
            max_cached_prime * from_u8(2).unwrap(),
            min + from_uint(MAX_SEGMENT_SIZE).unwrap()
        );

        let len = max - min;
        let uint_len = len.to_uint().unwrap();

        let mut values = Vec::with_capacity(uint_len);
        for value in range(min, min + len) {
            values.push(Some(value));
        }

        Segment {
            min: min,
            max: max,
            len: uint_len,
            values: values
        }
    }
}

#[test]
fn test_first_few_primes() {
    let first_primes = [2, 3, 5, 7, 11, 13, 17, 19, 23];
    let mut iter: Sieve<uint> = new();

    for &prime in first_primes.iter() {
        assert_eq!(iter.next(), Some(prime));
    }
}
