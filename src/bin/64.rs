/* Problem 64: Odd period square roots
 *
 * All square roots are periodic when written as continued fractions and can be written in the form:
 *
 * √N = a0 + 1 / (a1 + 1 / (a2 + 1 / (a3 + ... ) ) )
 *
 * For example, let us consider √23:
 *
 * √23 = 4 + √23 — 4 = 4 + 1 / 1 / (√23 - 4) = 4 + 1 / (1 + (√23 - 3) / 7)
 *
 * If we continue we would get the following expansion:
 *
 * a0 = 4, 1/(√23—4) = (√23+4)/7   = (√23+4)/7 = 1 + (√23—3)/7
 * a1 = 1, 7/(√23—3) = 7(√23+3)/14 = (√23+3)/2 = 3 + (√23—3)/2
 * a2 = 3, 2/(√23—3) = 2(√23+3)/14 = (√23+3)/7 = 1 + (√23—4)/7
 * a3 = 1, 7/(√23—4) = 7(√23+4)/7  = (√23+4)/1 = 8 + √23—4
 * a4 = 8, 1/(√23—4) = (√23+4)/7   = (√23+4)/7 = 1 + (√23—3)/7
 * a5 = 1, 7/(√23—3) = 7(√23+3)/14 = (√23+3)/2 = 3 + (√23—3)/2
 * a6 = 3, 2/(√23—3) = 2(√23+3)/14 = (√23+3)/7 = 1 + (√23—4)/7
 * a7 = 1, 7/(√23—4) = 7(√23+4)/7  = (√23+4)/4 = 8 + √23—4
 *
 * It can be seen that the sequence is repeating. For conciseness, we use the notation √23 =
 * [4;(1,3,1,8)], to indicate that the block (1,3,1,8) repeats indefinitely.
 *
 * The first ten continued fraction representations of (irrational) square roots are:
 *
 * √2=[1;(2)], period=1
 * √3=[1;(1,2)], period=2
 * √5=[2;(4)], period=1
 * √6=[2;(2,4)], period=2
 * √7=[2;(1,1,1,4)], period=4
 * √8=[2;(1,4)], period=2
 * √10=[3;(6)], period=1
 * √11=[3;(3,6)], period=2
 * √12= [3;(2,6)], period=2
 * √13=[3;(1,1,1,1,6)], period=5
 *
 * Exactly four continued fractions, for N ≤ 13, have an odd period.
 *
 * How many continued fractions for N ≤ 10000 have an odd period? */

use std::num::{ToPrimitive, Float};
use std::iter::count;
use std::collections::BTreeSet;

#[derive(Show, PartialEq)]
enum FractionType {
    Exact,
    Periodic(u32, Vec<u32>)
}

const MAX_N: u32 = 10_000;

#[cfg(not(test))]
fn main() {
    let result = range(2, MAX_N + 1).filter(|&n| {
        match divide_square(n) {
            FractionType::Exact => false,
            FractionType::Periodic(_, v) => (v.len() % 2 == 1),
        }
    }).count();

    println!("{}", result);
}

fn divide_square(number: u32) -> FractionType {
    enum SquareResult { Perfect, Floored(u32) }

    let closest_square_result = number.to_f32().and_then(|as_float| {
        let square_root = as_float.sqrt();

        if square_root.trunc() == square_root {
            Some(SquareResult::Perfect)
        } else {
            square_root.floor().to_u32().map(|num| {
                SquareResult::Floored(num)
            })
        }
    });

    let closest_square = match closest_square_result {
        Some(SquareResult::Perfect)      => return FractionType::Exact,
        Some(SquareResult::Floored(num)) => num,
        None                             => panic!("Couldn't square {}", number),
    };

    let mut period = Vec::new();

    let mut prev_step = Step {
        closest_square: closest_square,
        number: number,
        numerator: 1,
        rest: closest_square,
    };

    let mut seen_steps = BTreeSet::new();

    loop {
        let (step_result, next_step) = step(prev_step);

        if seen_steps.contains(&next_step) {
            break;
        }

        prev_step = next_step.clone();
        seen_steps.insert(next_step.clone());
        period.push(step_result);
    }

    return FractionType::Periodic(closest_square, period);
}

#[derive(Show, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Step { closest_square: u32, numerator: u32, number: u32, rest: u32 }

fn step(prev: Step) -> (u32, Step) {
    let numerator = (prev.number - (prev.rest * prev.rest)) / prev.numerator;

    let multiplier = count(0, 1).take_while(|&n| {
        n * numerator <= prev.closest_square + prev.rest
    }).last().unwrap();


    let new_rest = (multiplier * numerator) - prev.rest;

    (multiplier, Step {
        closest_square: prev.closest_square,
        number: prev.number,
        numerator: numerator,
        rest: new_rest,
    })
}

#[cfg(test)]
mod tests {
    use std::num::Float;
    use super::{step, divide_square, Step};
    use super::FractionType::{Exact, Periodic};

    #[test]
    fn test_example_divisions() {
        assert_eq!(divide_square(2),  Periodic(1, vec![2]));
        assert_eq!(divide_square(3),  Periodic(1, vec![1,2]));
        assert_eq!(divide_square(4),  Exact);
        assert_eq!(divide_square(5),  Periodic(2, vec![4]));
        assert_eq!(divide_square(6),  Periodic(2, vec![2,4]));
        assert_eq!(divide_square(7),  Periodic(2, vec![1,1,1,4]));
        assert_eq!(divide_square(8),  Periodic(2, vec![1,4]));
        assert_eq!(divide_square(9),  Exact);
        assert_eq!(divide_square(10), Periodic(3, vec![6]));
        assert_eq!(divide_square(11), Periodic(3, vec![3,6]));
        assert_eq!(divide_square(12), Periodic(3, vec![2,6]));
        assert_eq!(divide_square(13), Periodic(3, vec![1,1,1,1,6]));
    }

    fn def_step(numerator: u32, number: u32, rest: u32) -> Step {
        let closest_square = number.to_f32().and_then(|as_float| {
            as_float.sqrt().floor().to_u32()
        }).unwrap();

        Step { numerator: numerator, number: number, rest: rest, closest_square: closest_square }
    }

    #[test]
    fn test_division_steps() {
        assert_eq!(step(def_step(1, 23, 4)), (1, def_step(7, 23, 3)));
        assert_eq!(step(def_step(7, 23, 3)), (3, def_step(2, 23, 3)));
        assert_eq!(step(def_step(2, 23, 3)), (1, def_step(7, 23, 4)));
        assert_eq!(step(def_step(7, 23, 4)), (8, def_step(1, 23, 4)));
        assert_eq!(step(def_step(1, 23, 4)), (1, def_step(7, 23, 3)));
        assert_eq!(step(def_step(7, 23, 3)), (3, def_step(2, 23, 3)));
        assert_eq!(step(def_step(2, 23, 3)), (1, def_step(7, 23, 4)));
        assert_eq!(step(def_step(7, 23, 4)), (8, def_step(1, 23, 4)));
    }
}
