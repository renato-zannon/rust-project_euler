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

extern crate shared;

use shared::sieve;
use std::iter::count;
use std::collections::{HashMap, HashSet};

static CONSECUTIVE_COUNT: uint = 4;

fn main() {
  let mut sieve = sieve::new();
  let mut memo  = HashMap::new();

  let result = count(1u, 1).find(|&first_number| {
    range(first_number, first_number + CONSECUTIVE_COUNT).all(|number| {
      sieve.compute_until(number);

      let count = factors_for_number(FactorCount {
        remaining: number,
        memo:      &mut memo,
        primes:    sieve.found_primes(),
      });

      count == CONSECUTIVE_COUNT
    })
  }).unwrap();

  println!("{}", result);
}

type Memo<'a> = &'a mut HashMap<uint, HashSet<uint>>;

fn factors_for_number(count: FactorCount) -> uint {
  let (_, result) = factors(count);
  result.len()
}

struct FactorCount<'a> {
  remaining: uint,
  memo:      Memo<'a>,
  primes:    &'a [uint]
}

fn factors(mut count: FactorCount) -> (Memo, HashSet<uint>) {
  match count.memo.find_copy(&count.remaining) {
    Some(cached) => {
      return (count.memo, cached);
    },

    None => ()
  }

  let first_factor = count.primes.iter().find(|&&prime| {
    count.remaining % prime == 0 
  }).map(|value| *value);

  let (new_memo, result) = match first_factor {
    Some(factor) => {
      let subcount = FactorCount { remaining: count.remaining / factor, ..count };
      let (new_memo, mut result) = factors(subcount);

      result.insert(factor);
      (new_memo, result)
    },

    None => (count.memo, HashSet::new()),
  };

  count.memo = new_memo;
  count.memo.insert(count.remaining, result.clone());

  (count.memo, result)
}
