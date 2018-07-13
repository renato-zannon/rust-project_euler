/* Problem 46: Goldbach's other conjecture
 *
 * It was proposed by Christian Goldbach that every odd composite number can be written as the sum
 * of a prime and twice a square.
 *
 * 9 = 7 + 2×1²
 * 15 = 7 + 2×2²
 * 21 = 3 + 2×3²
 * 25 = 7 + 2×3²
 * 27 = 19 + 2×2²
 * 33 = 31 + 2×1²
 *
 * It turns out that the conjecture was false.
 *
 * What is the smallest odd composite that cannot be written as the sum of a prime and twice a
 * square? */

extern crate shared;

use shared::sieve;
use shared::sieve::Sieve as PrimeSieve;

const SEGMENT_SIZE: usize = 101;

fn main() {
    let mut sieve = sieve::new();

    for segment_start in (3..).step_by(OddNumberSegment::length() + 1) {
        let mut segment = new_segment(segment_start);
        mark_odd_composites(&mut segment, &mut sieve);

        for unmarked in segment.unmarked_numbers() {
            if !sieve.is_prime(unmarked) {
                println!("{}", unmarked);
                return;
            }
        }
    }
}

fn mark_odd_composites(segment: &mut OddNumberSegment, sieve: &mut PrimeSieve<usize>) {
    let segment_start = segment.first();
    let segment_end = segment.last();

    sieve.compute_until(segment_end);

    let primes = sieve.found_primes()[1..] // Skip 2, as 2 + 2*x² is always even
        .iter()
        .take_while(|prime| **prime < segment_end);

    for &prime in primes {
        let min_squared_number = min_half_square(prime, segment_start, Rounding::Up);
        let max_squared_number = min_half_square(prime, segment_end, Rounding::Down);

        for number in min_squared_number..max_squared_number + 1 {
            let result = prime + 2 * number * number;
            segment.mark_number(result);
        }
    }
}

enum Rounding {
    Up,
    Down,
}

fn min_half_square(prime: usize, end: usize, rounding: Rounding) -> usize {
    if prime >= end {
        0
    } else {
        let diff = (end - prime) as f64;
        let result = (diff / 2.0).sqrt();

        let rounded = match rounding {
            Rounding::Up => result.ceil(),
            Rounding::Down => result.floor(),
        };

        rounded as usize
    }
}

struct OddNumberSegment {
    values: [bool; SEGMENT_SIZE],
    start: usize,
}

fn new_segment(start: usize) -> OddNumberSegment {
    let end = start + SEGMENT_SIZE - 1;

    assert!(start % 2 == 1, "segment start is not odd: {}", start);
    assert!(end % 2 == 1, "segment end is not odd: {}", end);

    OddNumberSegment {
        values: [false; SEGMENT_SIZE],
        start: start,
    }
}

impl OddNumberSegment {
    fn mark_number(&mut self, number: usize) {
        let index = {
            let is_odd = number % 2 == 1;
            assert!(is_odd, "{} is not an odd number!", number);

            let in_range = number >= self.start && number <= self.last();
            assert!(
                in_range,
                "{} is out of the segment range [{}..{}]",
                number,
                self.start,
                self.last()
            );

            (number - self.start) / 2
        };

        self.values[index] = true;
    }

    fn unmarked_numbers(&self) -> Vec<usize> {
        self.values
            .iter()
            .enumerate()
            .filter_map(|(index, value)| {
                if *value {
                    None
                } else {
                    Some(index * 2 + self.start)
                }
            })
            .collect()
    }

    fn length() -> usize {
        SEGMENT_SIZE * 2 - 1
    }

    fn first(&self) -> usize {
        self.start
    }

    fn last(&self) -> usize {
        self.start + OddNumberSegment::length() - 1
    }
}
