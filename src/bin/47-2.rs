/* Problem 47: Distinct primes factors
 *
 * The first two consecutive numbers to have two distinct prime factors are:
 *
 * 14 = 2 × 7
 * 15 = 3 × 5
 *
 * The first three consecutive numbers to have three distinct prime factors are:
 *
 * 644 = 2² × 7 × 23
 * 645 = 3 × 5 × 43
 * 646 = 2 × 17 × 19.
 *
 * Find the first four consecutive integers to have four distinct prime factors. What is the first
 * of these numbers? */

#![feature(collections, core)]

extern crate shared;
use shared::sieve;
use std::iter::{range_step_inclusive, count};
use std::cmp::Ordering;

const SEGMENT_SIZE: usize = 1_000;
const FACTOR_COUNT: usize = 4;
const CONSECUTIVE_COUNT: usize = 4;

// Alternatvie, Dynamic programming-based implementation
fn main() {
    let mut sieve = sieve::new();

    for segment_start in count(2, SEGMENT_SIZE) {
        let mut segment = Segment::new(segment_start);
        let segment_end = segment.last_number();

        sieve.compute_until(segment_end);

        for prime in sieve.found_primes().iter().take_while(|&&p| p <= segment_end) {
            let prime = *prime;

            let first_composite = match prime.cmp(&segment_start) {
                Ordering::Equal | Ordering::Greater => prime,
                Ordering::Less => segment_start + (prime - (segment_start % prime)) % prime,
            };

            for composite in range_step_inclusive(first_composite, segment_end, prime) {
                segment.add_factor(composite, prime);
            }
        }

        let mut head = None;
        let mut consecutive_count = 0;

        for (number, factor_count) in segment.number_factors() {
            if factor_count == FACTOR_COUNT {
                head = head.or(Some(number));
                consecutive_count += 1;
            } else {
                head = None;
                consecutive_count = 0;
            }

            if consecutive_count == CONSECUTIVE_COUNT {
                println!("{}", head.unwrap());
                return;
            }
        }
    }
}

struct Segment {
    start: usize,
    values: [FactorCount; SEGMENT_SIZE],
}

#[derive(Copy)]
struct FactorCount {
    factors: [Option<usize>; FACTOR_COUNT],
    count: usize,
}

struct NumberFactors<'a> {
    start: usize,
    values: &'a [FactorCount],
}

impl Segment {
    fn new(start: usize) -> Segment {
        let factor_count = FactorCount {
            factors: [None; FACTOR_COUNT],
            count: 0,
        };

        Segment {
            start: start,
            values: [factor_count; SEGMENT_SIZE],
        }
    }

    fn add_factor(&mut self, number: usize, factor: usize) {
        let first = self.first_number();
        let last  = self.last_number();

        assert!(number >= first && number <= last,
            "Number {} outside allowed range - [{}, {}]", number, first, last);

        let factor_count = &mut self.values[number - first];
        factor_count.add_factor(factor);
    }

    fn number_factors(&self) -> NumberFactors {
        NumberFactors {
            start: self.start,
            values: &self.values[],
        }
    }

    fn first_number(&self) -> usize {
        self.start
    }

    fn last_number(&self) -> usize {
        self.start + SEGMENT_SIZE - 1
    }
}

impl FactorCount {
    fn add_factor(&mut self, factor: usize) {
        if self.count == FACTOR_COUNT {
            return;
        }

        for maybe_factor in self.factors.iter_mut() {
            match *maybe_factor {
                None => {
                    self.count += 1;
                    *maybe_factor = Some(factor);
                    return;
                },

                Some(fact) if fact == factor => return,
                _ => continue,
            }
        }
    }
}

impl<'a> Iterator for NumberFactors<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        match self.values.first() {
            Some(factor_count) => {
                let number = self.start;
                self.start += 1;
                self.values = self.values.tail();

                Some((number, factor_count.count))
            },

            None => None,
        }
    }
}
