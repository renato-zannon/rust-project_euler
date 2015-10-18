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

const MAX_N: usize     = 100;
const MIN_VALUE: usize = 1_000_000;

fn main() {
    let count = (1..MAX_N + 1).fold(0, |prev, n| {
        let mut value = 1usize;

        // Multiplicative formula from:
        // http://en.wikipedia.org/wiki/Binomial_coefficient#Multiplicative_formula
        let first_k = (1..n / 2 + 1).find(|&k| {
            let old_value = value;
            value = old_value * (n + 1 - k) / k;

            return value > MIN_VALUE;
        });

        // Since the binomial is symmetrical, we can easily know how many results > 1_000_000 are there,
        // given we know what the first one is.
        match first_k {
            Some(k) => prev + n + 1 - 2*k,
            None    => prev,
        }
    });

    println!("{}", count);
}
