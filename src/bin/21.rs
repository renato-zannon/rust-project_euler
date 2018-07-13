/* Problem 21: Amicable numbers
 *
 * Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly
 * into n).  If d(a) = b and d(b) = a, where a != b, then a and b are an amicable pair and each of a
 * and b are called amicable numbers.
 *
 * For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore
 * d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.
 *
 * Evaluate the sum of all the amicable numbers under 10000. */

extern crate num;

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

const MAX: u32 = 10_000;

fn main() {
    let mut divisor_sums: HashMap<u32, u32> = HashMap::with_capacity(MAX as usize);
    let mut amicables = HashSet::new();

    for num in 1..MAX {
        if amicables.contains(&num) {
            continue;
        }

        let div_sum = cached_divisor_sum(num, &mut divisor_sums);
        let reverse_sum = cached_divisor_sum(div_sum, &mut divisor_sums);

        if num == reverse_sum && div_sum != num {
            amicables.insert(num);
            amicables.insert(div_sum);
        }
    }

    let result: u32 = amicables.into_iter().fold(0, |result, num| num + result);
    println!("{}", result);
}

fn cached_divisor_sum(num: u32, cache: &mut HashMap<u32, u32>) -> u32 {
    match cache.entry(num) {
        Entry::Vacant(entry) => *entry.insert(divisor_sum(num)),
        Entry::Occupied(entry) => entry.remove(),
    }
}

fn divisor_sum(num: u32) -> u32 {
    let num_sqrt = (num as f64).sqrt() as u32;

    (2..num_sqrt + 1)
        .map(|candidate| {
            if num % candidate > 0 {
                0
            } else if candidate == num_sqrt {
                candidate
            } else {
                candidate + (num / candidate)
            }
        })
        .fold(1, |sum, term| sum + term)
}
