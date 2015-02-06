use std::collections::BTreeSet;
use std::num::Float;
use std::iter::count;

#[derive(Debug, PartialEq)]
pub enum FractionType {
    Exact,
    Periodic(u32, Vec<u32>)
}

pub fn divide_square(number: u32) -> FractionType {
    let closest_square: u32 = {
        let square_root = Float::sqrt(number as f32);

        if square_root.trunc() == square_root {
            return FractionType::Exact;
        }

        square_root.floor() as u32
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

    FractionType::Periodic(closest_square, period)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
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

// struct ConvergentIterator {
//     number: u32,
//     last_pair: Option<(u32, u32)>,
// }
//
// impl Iterator for ConvergentIterator {
//     type Item = (u32, u32);
//
//     fn next(&mut self) -> Option<(u32, u32)> {
//         let last_pair = match self.last_pair {
//             Some(pair) => pair,
//
//             None => {
//                 // k0 = 1 + (x - 1) / 2 = (x + 1)/2
//                 let pair = Some((self.number + 1, 2));
//                 self.last_pair = pair.clone();
//                 return pair;
//             }
//         };
//     }
// }

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
        let closest_square = Float::sqrt(number as f32).floor() as u32;

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
