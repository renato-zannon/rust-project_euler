/* Problem 74: Digit factorial chains
 *
 * The number 145 is well known for the property that the sum of the factorial of its digits is
 * equal to 145:
 *
 * 1! + 4! + 5! = 1 + 24 + 120 = 145
 *
 * Perhaps less well known is 169, in that it produces the longest chain of numbers that link
 * back to 169; it turns out that there are only three such loops that exist:
 *
 * 169 → 363601 → 1454 → 169
 * 871 → 45361 → 871
 * 872 → 45362 → 872
 *
 * It is not difficult to prove that EVERY starting number will eventually get stuck in a loop.
 * For example,
 *
 * 69 → 363600 → 1454 → 169 → 363601 (→ 1454)
 * 78 → 45360 → 871 → 45361 (→ 871)
 * 540 → 145 (→ 145)
 *
 * Starting with 69 produces a chain of five non-repeating terms, but the longest non-repeating
 * chain with a starting number below one million is sixty terms.
 *
 * How many chains, with a starting number below one million, contain exactly sixty non-repeating
 * terms? */



use fnv::{FnvHashMap, FnvHashSet};
use shared::digits;

const DIGIT_FACTORIALS: [u32; 10] = [
    1,      // 0!
    1,      // 1!
    2,      // 2!
    6,      // 3!
    24,     // 4!
    120,    // 5!
    720,    // 6!
    5040,   // 7!
    40320,  // 8!
    362880, // 9!
];

const MAX: u32 = 1_000_000;

fn main() {
    let mut digit_cache = FnvHashMap::default();
    digit_cache.reserve(MAX as usize);

    let result = (1..MAX)
        .map(|n| chain_size(n, &mut digit_cache))
        .filter(|n| *n == 60)
        .count();

    println!("{}", result);
}

fn chain_size(n: u32, cache: &mut FnvHashMap<u32, u32>) -> u32 {
    let mut seen = FnvHashSet::with_capacity_and_hasher(60, Default::default());
    let mut current = n;

    while seen.insert(current) {
        current = *cache
            .entry(current)
            .or_insert_with(|| factorial_digit_sum(current));
    }

    seen.len() as u32
}

fn factorial_digit_sum(n: u32) -> u32 {
    digits::new(n).map(|d: usize| DIGIT_FACTORIALS[d]).sum()
}
