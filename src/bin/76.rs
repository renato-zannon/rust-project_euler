/* Problem 76: Counting summations
 *
 * It is possible to write five as a sum in exactly six different ways:
 *
 * 4 + 1
 * 3 + 2
 * 3 + 1 + 1
 * 2 + 2 + 1
 * 2 + 1 + 1 + 1
 * 1 + 1 + 1 + 1 + 1
 *
 * How many different ways can one hundred be written as a sum of at least two positive integers?
 **/

use rayon::prelude::*;
use smallvec::{smallvec, SmallVec};
use std::collections::{HashSet, VecDeque};

fn main() {
    println!("{}", ways_to_make(70));
}

fn ways_to_make(n: i32) -> usize {
    let mut to_expand: VecDeque<SmallVec<[u8; 32]>> = VecDeque::new();

    for p1 in 1..=(n / 2) {
        let p2 = n - p1;
        to_expand.push_front(smallvec![p2 as u8, p1 as u8]);
    }

    let mut ways: HashSet<_> = to_expand.iter().cloned().collect();

    let mut new_sums_buffer = Vec::new();
    let mut last_p2 = 0;
    let mut last_te_p2 = 0;

    while let Some(sum) = to_expand.pop_front() {
        let new_sums = sum.par_iter().enumerate().flat_map(|(index, &part)| {
            let sum = &sum;
            let ways = &ways;

            (1..part).into_par_iter().filter_map(move |substitute| {
                let mut new_sum = sum.clone();
                new_sum[index] = part - substitute;
                new_sum.push(substitute);
                new_sum.par_sort_unstable_by_key(|sum_part| std::u8::MAX - sum_part);

                if ways.contains(&new_sum) {
                    return None;
                }

                return Some(new_sum);
            })
        });

        new_sums_buffer.par_extend(new_sums);
        for new_sum in new_sums_buffer.drain(..) {
            ways.insert(new_sum.clone());
            to_expand.push_front(new_sum);
        }

        let ways_p2 = (ways.len() as f32).log2().trunc() as i32;
        let te_p2 = (to_expand.len() as f32).log2().trunc() as i32;

        if ways_p2 > last_p2 || te_p2 != last_te_p2 {
            last_p2 = ways_p2;
            last_te_p2 = te_p2;

            println!("Ways: {} (log2 {})", ways.len(), ways_p2,);
            println!("Left to expand: {} (log2 {})", to_expand.len(), te_p2,);
            println!("----");
        }
    }

    return ways.len();
}
