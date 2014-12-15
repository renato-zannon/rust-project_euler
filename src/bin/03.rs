/* Problem 3: Largest prime factor
 *
 * The prime factors of 13195 are 5, 7, 13 and 29.
 * What is the largest prime factor of the number 600851475143 ? */

use std::iter::Unfold;

fn main() {
    let factors = Unfold::new((600851475143, 2), unfold_factors);
    println!("{}", factors.max());
}

fn unfold_factors(state_ptr : &mut (uint, uint)) -> Option<uint> {
    let (remaining, divisor) = *state_ptr;

    if remaining <= 1 {
        return None;
    }

    let mut new_divisor = divisor;
    while remaining % new_divisor > 0 {
        new_divisor += 1;
    }

    *state_ptr = (remaining / new_divisor, new_divisor);
    Some(new_divisor)
}
