/* Problem 53: Combinatoric selections
 *
 * There are exactly ten ways of selecting three from five, 12345:
 *
 * 123, 124, 125, 134, 135, 145, 234, 235, 245, and 345
 *
 * In combinatorics, we use the notation, ⁵C₃ = 10.
 *
 * In general,
 * nCr = n!/r!(n−r)!
 *
 * where r ≤ n, n! = n×(n−1)×...×3×2×1, and 0! = 1.
 *
 * It is not until n = 23, that a value exceeds one-million: ²³C₁₀ = 1144066.
 *
 * How many, not necessarily distinct, values of  nCr, for 1 ≤ n ≤ 100, are greater than
 * one-million? */

use std::iter::AdditiveIterator;
use std::iter::range_inclusive;

static MAX_N: uint     = 100;
static MIN_VALUE: uint = 1_000_000;

fn main() {
  let count = range_inclusive(1u, MAX_N).map(|n| {
    let mut value = 1u;

    // Multiplicative formula from:
    // http://en.wikipedia.org/wiki/Binomial_coefficient#Multiplicative_formula
    let first_k = range_inclusive(1u, n / 2).find(|&k| {
      let old_value = value;
      value = old_value * (n + 1 - k) / k;

      return value > MIN_VALUE;
    });

    // Since the binomial is symmetrical, we can easily know how many results > 1_000_000 are there,
    // given we know what the first one is.
    match first_k {
      Some(k) => n + 1 - 2*k,
      None    => 0,
    }
  }).sum();

  println!("{}", count);
}
