/* Problem 68: Magic 5-gon ring
 *
 * Consider the following "magic" 3-gon ring, filled with the numbers 1 to 6, and each line adding to nine.
 *
 * Working clockwise, and starting from the group of three with the numerically lowest external node (4,3,2 in this example),
 * each solution can be described uniquely. For example, the above solution can be described by the set: 4,3,2; 6,2,1; 5,1,3.
 *
 * It is possible to complete the ring with four different totals: 9, 10, 11, and 12. There are eight solutions in total.
 * Total  Solution Set
 * 9      4,2,3; 5,3,1; 6,1,2
 * 9      4,3,2; 6,2,1; 5,1,3
 * 10     2,3,5; 4,5,1; 6,1,3
 * 10     2,5,3; 6,3,1; 4,1,5
 * 11     1,4,6; 3,6,2; 5,2,4
 * 11     1,6,4; 5,4,2; 3,2,6
 * 12     1,5,6; 2,6,4; 3,4,5
 * 12     1,6,5; 3,5,4; 2,4,6
 *
 * By concatenating each group it is possible to form 9-digit strings; the maximum string for a 3-gon ring is 432621513.
 *
 * Using the numbers 1 to 10, and depending on arrangements, it is possible to form 16- and 17-digit strings.
 * What is the maximum 16-digit string for a "magic" 5-gon ring? */
use itertools;

use shared::Permutations;
use smallvec::SmallVec;
use std::fmt::Write;

const RING_WIDTH: usize = 5;
const LINE_LENGTH: usize = 3;
const MAX_NUMBER: u8 = 10;
const MAX_SOLUTION_LENGTH: usize = 16;

fn main() {
    let max_solution = solutions()
        .filter(below_max_length)
        .max_by(|s1, s2| {
            use std::cmp::Ordering::{self, Equal};

            s1.iter()
                .zip(s2)
                .map(|(n1, n2)| n1.cmp(n2))
                .fold(Equal, Ordering::then)
        })
        .unwrap();

    let mut result = String::with_capacity(MAX_SOLUTION_LENGTH);
    for set in max_solution {
        for number in set {
            write!(&mut result, "{}", number).unwrap();
        }
    }

    println!("{}", result);
}

fn below_max_length(solution: &Solution) -> bool {
    let mut len = 0;

    for set in solution {
        for num in set {
            if *num < 10 {
                len += 1
            } else {
                len += 2;
            }
        }
    }

    len <= MAX_SOLUTION_LENGTH
}

type SolutionPart = SmallVec<[u8; LINE_LENGTH]>;
type Solution = SmallVec<[SolutionPart; RING_WIDTH]>;

fn solutions() -> impl Iterator<Item = Solution> {
    let digits: SmallVec<[u8; MAX_NUMBER as usize + 1]> = (1..=MAX_NUMBER).collect();

    itertools::unfold(digits.permutations(), |state| {
        state.permute().map(|s| maybe_solution(s))
    })
    .filter_map(|opt| opt)
}

fn maybe_solution(numbers: &[u8]) -> Option<Solution> {
    let (external, others) = numbers.split_at(RING_WIDTH);

    if external.iter().min() != external.first() {
        return None;
    }

    let mut sets: Solution = external
        .iter()
        .map(|&e| {
            let mut v = SolutionPart::new();
            v.push(e);
            v
        })
        .collect();

    for n in 1..LINE_LENGTH {
        for (set_index, set) in sets.iter_mut().enumerate() {
            let index = (n + set_index) % others.len();
            set.push(others[index]);
        }
    }

    if is_solution(&sets) {
        Some(sets)
    } else {
        None
    }
}

fn is_solution(sets: &Solution) -> bool {
    let mut seen_sum: Option<u8> = None;

    for set in sets.into_iter() {
        let sum = set.into_iter().sum();

        match seen_sum {
            None => seen_sum = Some(sum),
            Some(s) if s != sum => return false,
            _ => {}
        }
    }

    true
}
