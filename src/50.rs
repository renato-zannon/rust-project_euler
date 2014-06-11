/* Problem 50: Consecutive prime sum
 *
 * The prime 41, can be written as the sum of six consecutive primes:
 * 41 = 2 + 3 + 5 + 7 + 11 + 13
 *
 * This is the longest sum of consecutive primes that adds to a prime below one-hundred.
 *
 * The longest sum of consecutive primes below one-thousand that adds to a prime, contains 21 terms,
 * and is equal to 953.
 *
 * Which prime, below one-million, can be written as the sum of the most consecutive primes? */

extern crate shared;
use shared::sieve;
use shared::sieve::Sieve;
use std::iter::AdditiveIterator;

static MAX_PRIME: uint = 1_000_000;


fn main() {
  let primes: Vec<uint> = sieve::new()
    .take_while(|&prime| prime < MAX_PRIME)
    .collect();

  let mut sieve = sieve::new();

  let mut sequence = PrimeSequenceSum {
    value: primes.iter().map(|prime| *prime).sum(),
    start_index: 0,
    end_index:   primes.len() - 1,
    all_primes: primes.as_slice()
  };

  // Reduce the sequence until we find the longest possible that begins on the first prime
  while !suitable_prime(sequence.value, &mut sieve) {
    sequence = sequence.shrink_right().unwrap()
  }

  // Try bigger sequences by starting the sequence on the second, third etc prime
  sequence = longest_after_advancing(sequence, &mut sieve);
  println!("{}", sequence.value);
}

fn suitable_prime(value: uint, sieve: &mut Sieve) -> bool {
  value < MAX_PRIME && sieve.is_prime(value)
}

fn longest_after_advancing<'a>(seq: PrimeSequenceSum<'a>, sieve: &mut Sieve) -> PrimeSequenceSum<'a> {
  let mut longest_sequence = seq.clone();
  let mut prev_try = seq.clone();

  loop {
    let try = match prev_try.advance() {
      None           => break,
      Some(advanced) => longest_after_expanding(advanced, sieve)
    };

    if try.value > MAX_PRIME { break; }

    if try.len() > longest_sequence.len() {
      longest_sequence = try.clone();
    }

    prev_try = try;
  }

  longest_sequence
}

fn longest_after_expanding<'a>(seq: PrimeSequenceSum<'a>, sieve: &mut Sieve) -> PrimeSequenceSum<'a> {
  let mut longest = seq.clone();
  let mut longest_prime = seq;

  while longest.value < MAX_PRIME {
    match longest.expand_left() {
      Some(advanced) => {
        if sieve.is_prime(advanced.value) {
          longest_prime = advanced.clone();
        }

        longest = advanced;
      },

      None => break
    }
  }

  longest_prime
}

#[deriving(Show, Clone)]
struct PrimeSequenceSum<'a> {
  value:  uint,

  start_index: uint,
  end_index:   uint,
  all_primes:  &'a [uint],
}

impl<'a> PrimeSequenceSum<'a> {
  fn len(&self) -> uint {
    self.end_index - self.start_index + 1
  }

  fn expand_left(&self) -> Option<PrimeSequenceSum<'a>> {
    self.peek_left().map(|(prev_index, prev_prime)| {
      PrimeSequenceSum {
        value:       self.value + prev_prime,
        start_index: prev_index,
        ..*self
      }
    })
  }

  fn advance(&self) -> Option<PrimeSequenceSum<'a>> {
    self.peek_right().map(|(next_index, next_prime)| {
      let first_on_sequence = self.all_primes[self.start_index];

      PrimeSequenceSum {
        value:       self.value + next_prime - first_on_sequence,
        start_index: self.start_index + 1,
        end_index:   next_index,
        ..*self
      }
    })
  }

  fn shrink_right(&self) -> Option<PrimeSequenceSum<'a>> {
    if self.start_index == self.end_index { return None; }

    let last_on_sequence = self.all_primes[self.end_index];
    Some(PrimeSequenceSum {
      value:     self.value - last_on_sequence,
      end_index: self.end_index - 1,
      ..*self
    })
  }

  fn peek_left(&self) -> Option<(uint, uint)> {
    match self.start_index {
      0     => None,
      index => Some((index - 1, self.all_primes[index - 1]))
    }
  }

  fn peek_right(&self) -> Option<(uint, uint)> {
    if self.end_index >= self.all_primes.len() - 1 {
      None
    } else {
      let next_index = self.end_index + 1;
      Some((next_index, self.all_primes[next_index]))
    }
  }
}
