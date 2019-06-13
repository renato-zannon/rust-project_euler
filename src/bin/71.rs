/* Problem 71: Ordered fractions
 *
 * Consider the fraction, n/d, where n and d are positive integers. If n<d and HCF(n,d)=1, it is
 * called a reduced proper fraction.
 *
 * If we list the set of reduced proper fractions for d ≤ 8 in ascending order of size, we get:
 *
 * 1/8, 1/7, 1/6, 1/5, 1/4, 2/7, 1/3, 3/8, 2/5, 3/7, 1/2, 4/7, 3/5, 5/8, 2/3, 5/7, 3/4, 4/5, 5/6, 6/7, 7/8
 *
 * It can be seen that 2/5 is the fraction immediately to the left of 3/7.
 *
 * By listing the set of reduced proper fractions for d ≤ 1,000,000 in ascending order of size,
 * find the numerator of the fraction immediately to the left of 3/7. */

use num::rational::Ratio;
use rayon::prelude::*;

type Int = u64;
const MAX: Int = 1_000_000;

fn main() {
    let target_ratio: Ratio<Int> = Ratio::new(3, 7);

    let result = (1..MAX)
        .into_par_iter()
        .flat_map(|denom| {
            let midpoint = (target_ratio * denom).ceil().to_integer();
            let start = midpoint.saturating_sub(1);

            (start..midpoint)
                .into_par_iter()
                .map(move |num| (num, denom))
        })
        .filter_map(|(num, denom)| {
            if denom == 0 || denom > MAX {
                return None;
            }

            let ratio = Ratio::new(num, denom);

            if ratio >= target_ratio {
                None
            } else {
                Some(ratio)
            }
        })
        .max();

    println!("{:?}", result);
}
