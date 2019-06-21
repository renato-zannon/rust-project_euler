/* Problem 87: Prime power triples
 *
 * The smallest number expressible as the sum of a prime square, prime cube, and prime fourth power
 * is 28. In fact, there are exactly four numbers below fifty that can be expressed in such a way:
 *
 * 28 = 2² + 2³ + 2⁴
 * 33 = 3² + 2³ + 2⁴
 * 49 = 5² + 2³ + 2⁴
 * 47 = 2² + 3³ + 2⁴
 *
 * How many numbers below fifty million can be expressed as the sum of a prime square, prime cube,
 * and prime fourth power?
 **/

use hashbrown::HashSet;
use shared::sieve;

const TARGET: u64 = 50_000_000;

fn main() {
    let candidate_primes: Vec<u64> = sieve::new()
        .take_while(|p| (p * p) + 8 + 16 <= TARGET)
        .collect();

    let result: HashSet<_> = powers(&candidate_primes, 2)
        .flat_map(|square| {
            powers(&candidate_primes, 3)
                .map(move |cube| square + cube)
                .take_while(move |&sum| sum <= TARGET)
        })
        .flat_map(|prev_sum| {
            powers(&candidate_primes, 4)
                .map(move |fourth| prev_sum + fourth)
                .take_while(move |&sum| sum < TARGET)
        })
        .collect();

    println!("{}", result.len());
}

fn powers<'p>(primes: &'p [u64], power: u32) -> impl Iterator<Item = u64> + 'p {
    primes.iter().map(move |p| p.pow(power))
}
