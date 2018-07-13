/* Problem 63: Powerful digit counts
 *
 * The 5-digit number, 16807=7⁵, is also a fifth power. Similarly, the 9-digit
 * number, 134217728=8⁹, is a ninth power.
 *
 * How many n-digit positive integers exist which are also an nth power? */

extern crate num;
use num::ToPrimitive;

fn main() {
    let mut count = 0u8;

    // x^n, for x >= 10, always has more than n digits, so the maximum base is 9
    for base in 1u8..10 {
        // Solving 10^(x-1) <= b^x < 10^x gives x <= 1 / (1 - log10(b))
        let max_exp = base
            .to_f32()
            .and_then(|as_float| {
                let base_log10 = as_float.log10();
                (1.0 - base_log10).recip().floor().to_u8()
            })
            .unwrap();

        count += max_exp;
    }

    println!("{}", count);
}
