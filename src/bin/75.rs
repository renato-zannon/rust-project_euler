/* Problem 75: Singular integer right triangles
 *
 * It turns out that 12 cm is the smallest length of wire that can be bent to form an integer sided
 * right angle triangle in exactly one way, but there are many more examples.
 *
 * 12 cm: (3,4,5)
 * 24 cm: (6,8,10)
 * 30 cm: (5,12,13)
 * 36 cm: (9,12,15)
 * 40 cm: (8,15,17)
 * 48 cm: (12,16,20)
 *
 * In contrast, some lengths of wire, like 20 cm, cannot be bent to form an integer sided right
 * angle triangle, and other lengths allow more than one solution to be found; for example, using
 * 120 cm it is possible to form exactly three different integer sided right angle triangles.
 *
 * 120 cm: (30,40,50), (20,48,52), (24,45,51)
 *
 * Given that L is the length of the wire, for how many values of L ≤ 1,500,000 can exactly one
 * integer sided right angle triangle be formed?
 **/


use num::Integer;

use std::collections::HashMap;
use std::iter;

const MAX_LENGTH: i32 = 1_500_000;

fn main() {
    let mut found_lengths = HashMap::new();
    let triplets = triplets();

    for triplet in triplets {
        let sum = triplet.0 + triplet.1 + triplet.2;

        found_lengths
            .entry(sum)
            .and_modify(|c| *c = true)
            .or_insert(false);
    }

    let result = found_lengths.values().filter(|&repeated| !repeated).count();
    println!("{}", result);
}

fn triplets() -> impl Iterator<Item = (i32, i32, i32)> {
    coprime_pairs().flat_map(|(m, n)| {
        triplets_from_pair(m, n)
    })
}

fn triplets_from_pair(m: i32, n: i32) -> impl Iterator<Item = (i32, i32, i32)> {
    let m2 = m * m;
    let n2 = n * n;
    let mut k = 1;

    iter::from_fn(move || {
        let a = k * (m2 - n2);
        let b = k * 2 * m * n;
        let c = k * (m2 + n2);

        if a + b + c > MAX_LENGTH {
            return None;
        } else {
            k += 1;
        }

        Some((a, b, c))
    })
}

fn coprime_pairs() -> impl Iterator<Item = (i32, i32)> {
    let mut m = 1;
    let mut n = 0;

    iter::from_fn(move || {
        loop {
            if n + 1 < m {
                n += 1;
            } else if (2*m*m + 2*m) < MAX_LENGTH - 1 {
                m += 1;
                n = 1;
            } else {
                return None;
            }

            let coprimes = m.gcd(&n) == 1;
            let both_odd = (m % 2 == 1) && (n % 2 == 1);

            if coprimes && !both_odd {
                break;
            }
        }

        Some((m, n))
    })
}
