/* Problem 66: Diophantine equation
 *
 * Consider quadratic Diophantine equations of the form:
 *
 * x² – Dy² = 1
 *
 * For example, when D=13, the minimal solution in x is 6492 – 13×1802 = 1.
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

use std::collections::BTreeSet;
use std::u64;

const MAX_D: u64 = 1000;

// x² - Dy² = 1
// x² - 1 = Dy²
// D = (x² - 1) / y²
// D = (x + 1)(x - 1) / y²

// x² - Dy² = 1
// x² = Dy² + 1

struct Solution {
    d: u64,
    x: u64,
}

fn main() {
    let mut ds_to_solve: BTreeSet<u64> = (1..MAX_D).collect();
    let mut ds_to_remove = Vec::with_capacity(MAX_D as usize);

    for root in 1..((MAX_D as f64).sqrt().ceil() as u64) {
        ds_to_solve.remove(&(root * root));
    }

    let max_y = (u64::MAX as f64).sqrt().ceil() as u64;
    let mut solution = Solution { d: 0, x: 0 };

    // D = (x² - 1) / y²
    // y²D = x² - 1
    // x² = y²D + 1
    // xx = yyD + 1
    for y in 2..max_y {
        let y_squared = y * y;

        for &d in ds_to_solve.iter() {
            let x_squared_candidate = (d * y_squared) + 1;

            match integer_root(x_squared_candidate) {
                Some(x) => {
                    println!("{:?} - {}", ds_to_solve, d);
                    ds_to_remove.push(d);

                    if x > solution.x {
                        solution = Solution { x: x, d: d };
                    }
                }

                None => continue,
            }
        }

        for d_to_remove in ds_to_remove.drain(..) {
            ds_to_solve.remove(&d_to_remove);
        }

        if ds_to_solve.len() == 0 {
            break;
        }
    }

    println!("{}", solution.d);
}

fn integer_root(_x: u64) -> Option<u64> {
    None
}
