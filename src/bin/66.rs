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
extern crate num;
extern crate shared;

use num::{BigUint, CheckedMul};
use shared::continued_fraction::convergent_iterator;
use std::collections::BTreeSet;

const MAX_D: u64 = 1000;

#[derive(Debug)]
struct Solution {
    d: BigUint,
    x: BigUint,
}

fn main() {
    let mut ds_to_solve: BTreeSet<BigUint> = (1..MAX_D).map(BigUint::from).collect();

    for root in 1..((MAX_D as f64).sqrt().ceil() as u64) {
        ds_to_solve.remove(&BigUint::from(root * root));
    }

    let mut solution = Solution {
        d: BigUint::from(0u32),
        x: BigUint::from(0u32),
    };

    for d in ds_to_solve {
        for pair in convergent_iterator(d.clone()) {
            let x = pair.numerator;
            let y = pair.denominator;

            let numerator_squared = x.checked_mul(&x).unwrap();
            let denominator_squared = y.checked_mul(&y).unwrap();

            if denominator_squared * &d == numerator_squared - 1u32 {
                println!("d = {}; x = {}", d, x);

                if x > solution.x {
                    solution = Solution { x, d };
                }

                break;
            }
        }
    }

    println!("{:?}", solution);
}
