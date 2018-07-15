/* Problem 66: Diophantine equation
 *
 * Consider quadratic Diophantine equations of the form:
 *
 * x² – Dy² = 1
 *
 * For example, when D=13, the minimal solution in x is 649^2 – 13×180^2 = 1.
 *
 * It can be assumed that there are no solutions in positive integers when D is square.
 *
 * By finding minimal solutions in x for D = {2, 3, 5, 6, 7}, we obtain the following:
 *
 * 3² – 2×2² = 1
 * 2² – 3×1² = 1
 * 9² – 5×4² = 1
 * 5² – 6×2² = 1
 * 8² – 7×3² = 1
 *
 * Hence, by considering minimal solutions in x for D ≤ 7, the largest x is obtained when D=5.
 *
 * Find the value of D ≤ 1000 in minimal solutions of x for which the largest value of x is
 * obtained. */
extern crate shared;

use shared::continued_fraction::convergent_iterator;
use std::collections::HashSet;

const MAX_D: u128 = 1000;

#[derive(Debug)]
struct Solution {
    d: u128,
    x: u128,
}

fn main() {
    let perfect_square_ds: HashSet<u128> = {
        let max_d_root = (MAX_D as f64).sqrt().ceil() as u128;

        (1..max_d_root).map(|root| root * root).collect()
    };

    let mut solution = Solution { d: 0, x: 0 };

    for d in 1..MAX_D {
        if perfect_square_ds.contains(&d) {
            continue;
        }

        for pair in convergent_iterator(d.clone()) {
            let x = pair.numerator;
            let y = pair.denominator;

            let numerator_squared = x.wrapping_mul(x);
            let denominator_squared = y.wrapping_mul(y);

            if denominator_squared.wrapping_mul(d) == numerator_squared - 1 {
                if x > solution.x {
                    solution = Solution { x, d };
                }

                break;
            }
        }
    }

    println!("{:?}", solution);
}
