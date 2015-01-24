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

use std::collections::{HashMap, BTreeSet};
use std::collections::hash_map::Entry;
use num::Integer;
use std::iter::range_inclusive as irange;
use std::iter::AdditiveIterator;
use std::num::Float;

fn main() {
    let mut divisor_sums: HashMap<u32, u32> = HashMap::new();
    let mut amicables = BTreeSet::new();

    for num in 1u32..10_000 {
        if amicables.contains(&num) { continue; }

        let sum = match divisor_sums.entry(num) {
            Entry::Vacant(entry)   => * entry.insert(divisor_sum(num)),
            Entry::Occupied(entry) => entry.remove(),
        };

        let reverse_sum = match divisor_sums.entry(sum) {
            Entry::Vacant(entry)   => * entry.insert(divisor_sum(sum)),
            Entry::Occupied(entry) => entry.remove(),
        };

        if num == reverse_sum && sum != num {
            amicables.insert(num);
            amicables.insert(sum);
        }
    }

    println!("{}", amicables.iter().map(|&x| x).sum());
}

fn divisor_sum(num: u32) -> u32 {
    let num_sqrt = (num as f64).sqrt() as u32;

    irange(2, num_sqrt).fold(1, |sum, candidate| {
        let (divided, remainder) = num.div_rem(&candidate);
        if remainder > 0 { return sum; }

        if candidate != divided {
            sum + candidate + divided
        } else {
            sum + candidate
        }
    })
}
