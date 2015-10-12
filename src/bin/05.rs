/* Problem 5: Smallest multiple
 *
 * 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any
 * remainder.
 *
 * What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20? */

use std::cmp::Ordering;

fn main() {
    let n = (1u64..21).fold(1, lcm);
    println!("{}", n);
}

fn lcm(n1: u64, n2: u64) -> u64 {
    let mut n1_factor = 1;
    let mut n2_factor = 1;

    loop {
        let n1_multiple = n1 * n1_factor;
        let n2_multiple = n2 * n2_factor;

        match n1_multiple.cmp(&n2_multiple) {
            Ordering::Less    => n1_factor += 1,
            Ordering::Greater => n2_factor += 1,
            Ordering::Equal   => return n1_multiple,
        }
    }
}
