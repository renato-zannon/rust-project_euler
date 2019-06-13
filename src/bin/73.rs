/* Problem 73: Counting fractions in a range
 *
 * Consider the fraction, n/d, where n and d are positive integers. If n<d and HCF(n,d)=1,
 * it is called a reduced proper fraction.
 *
 * If we list the set of reduced proper fractions for d ≤ 8 in ascending order of size,
 * we get:
 *
 * 1/8, 1/7, 1/6, 1/5, 1/4, 2/7, 1/3, 3/8, 2/5, 3/7, 1/2, 4/7, 3/5, 5/8, 2/3, 5/7, 3/4, 4/5,
 * 5/6, 6/7, 7/8
 *
 * It can be seen that there are 3 fractions between 1/3 and 1/2.
 *
 * How many fractions lie between 1/3 and 1/2 in the sorted set of reduced proper fractions
 * for d ≤ 12,000? */



use num::Integer;
use rayon::prelude::*;

use std::cmp;

const MAX: u64 = 12_000;

// 1/3 < a/b < 1/2
// 3 > b/a > 2
// 3a > b > 2a
fn main() {
    let count = (1..MAX)
        .into_par_iter()
        .flat_map(|a| {
            let min_b = (2 * a) + 1;
            let max_b = cmp::min((3 * a) - 1, MAX);

            let range = if min_b > MAX { 0..0 } else { min_b..max_b + 1 };

            range.into_par_iter().map(move |b| (a, b))
        })
        .filter(|(a, b)| a.gcd(&b) == 1)
        .count();

    println!("{}", count);
}
