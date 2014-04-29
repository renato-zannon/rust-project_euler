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

extern crate collections;
extern crate num;

use collections::hashmap::HashMap;
use collections::treemap::TreeSet;
use num::Integer;
use irange = std::iter::range_inclusive;
use std::iter::AdditiveIterator;

fn main() {
  let mut divisor_sums: HashMap<uint, uint> = HashMap::new();
  let mut amicables = TreeSet::new();

  for num in range(1u, 10_000) {
    if amicables.contains(&num) { continue; }

    let &sum         = divisor_sums.find_or_insert_with(num, divisor_sum);
    let &reverse_sum = divisor_sums.find_or_insert_with(sum, divisor_sum);

    if num == reverse_sum && sum != num {
      amicables.insert(num);
      amicables.insert(sum);
    }
  }

  println!("{}", amicables.iter().map(|&x| x).sum());
}

fn divisor_sum(&num: &uint) -> uint {
  let num_sqrt = (num as f64).sqrt() as uint;

  irange(2u, num_sqrt).fold(1, |sum, candidate| {
    let (divided, remainder) = num.div_rem(&candidate);
    if remainder > 0 { return sum; }

    if candidate != divided {
      sum + candidate + divided
    } else {
      sum + candidate
    }
  })
}
