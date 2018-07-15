use num::{self, Float, FromPrimitive, Integer, ToPrimitive};
use std::collections::BTreeSet;

#[derive(Debug, PartialEq)]
pub enum FractionType {
    Exact,
    Periodic(u32, Vec<u32>),
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
struct Step {
    closest_square: u32,
    numerator: u32,
    number: u32,
    rest: u32,
}

fn step(prev: Step) -> (u32, Step) {
    let numerator = (prev.number - (prev.rest * prev.rest)) / prev.numerator;

    let multiplier = (0..)
        .step_by(1)
        .take_while(|&n| n * numerator <= prev.closest_square + prev.rest)
        .last()
        .unwrap();

    let new_rest = (multiplier * numerator) - prev.rest;

    (
        multiplier,
        Step {
            closest_square: prev.closest_square,
            number: prev.number,
            numerator: numerator,
            rest: new_rest,
        },
    )
}

#[derive(Clone, Copy, Debug)]
pub struct ConvergentPair<T> {
    pub numerator: T,
    pub denominator: T,
}

pub struct ConvergentIterator<T> {
    period: Vec<T>,
    index: usize,
    next_a: T,
    prev_pair: ConvergentPair<T>,
    pprev_pair: ConvergentPair<T>,
}

pub fn convergent_iterator<T>(number: T) -> ConvergentIterator<T>
where
    T: ToPrimitive + From<u32>,
{
    match divide_square(number.to_u32().unwrap()) {
        FractionType::Exact => unimplemented!(),

        FractionType::Periodic(n, period) => {
            let period = period.into_iter().map(From::from).collect();

            ConvergentIterator {
                index: 0,
                period,
                next_a: From::from(n),
                prev_pair: ConvergentPair {
                    numerator: From::from(1),
                    denominator: From::from(0),
                },

                pprev_pair: ConvergentPair {
                    numerator: From::from(0),
                    denominator: From::from(1),
                },
            }
        }
    }
}

impl<T> Iterator for ConvergentIterator<T>
where
    T: Clone + num::CheckedMul + num::CheckedAdd,
{
    type Item = ConvergentPair<T>;

    fn next(&mut self) -> Option<ConvergentPair<T>> {
        use std::mem::swap;

        let pair = {
            let a = &self.next_a;

            let numerator = self.prev_pair
                .numerator
                .checked_mul(a)
                .and_then(|m| m.checked_add(&self.pprev_pair.numerator));

            let denominator = self.prev_pair
                .denominator
                .checked_mul(a)
                .and_then(|m| m.checked_add(&self.pprev_pair.denominator));

            match (numerator, denominator) {
                (Some(n), Some(d)) => ConvergentPair {
                    numerator: n,
                    denominator: d,
                },

                _ => return None,
            }
        };

        self.next_a = self.period[self.index % self.period.len()].clone();
        self.index += 1;
        swap(&mut self.pprev_pair, &mut self.prev_pair);
        self.prev_pair = pair.clone();

        Some(pair)
    }
}

#[cfg(test)]
mod tests {
    use super::FractionType::{Exact, Periodic};
    use super::{divide_square, step, Step};
    use num::Float;

    #[test]
    fn test_example_divisions() {
        assert_eq!(divide_square(2), Periodic(1, vec![2]));
        assert_eq!(divide_square(3), Periodic(1, vec![1, 2]));
        assert_eq!(divide_square(4), Exact);
        assert_eq!(divide_square(5), Periodic(2, vec![4]));
        assert_eq!(divide_square(6), Periodic(2, vec![2, 4]));
        assert_eq!(divide_square(7), Periodic(2, vec![1, 1, 1, 4]));
        assert_eq!(divide_square(8), Periodic(2, vec![1, 4]));
        assert_eq!(divide_square(9), Exact);
        assert_eq!(divide_square(10), Periodic(3, vec![6]));
        assert_eq!(divide_square(11), Periodic(3, vec![3, 6]));
        assert_eq!(divide_square(12), Periodic(3, vec![2, 6]));
        assert_eq!(divide_square(13), Periodic(3, vec![1, 1, 1, 1, 6]));
    }

    fn def_step(numerator: u32, number: u32, rest: u32) -> Step {
        let closest_square = Float::sqrt(number as f32).floor() as u32;

        Step {
            numerator: numerator,
            number: number,
            rest: rest,
            closest_square: closest_square,
        }
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
