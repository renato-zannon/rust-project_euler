/* Problem 60: Prime pair sets
 *
 * The primes 3, 7, 109, and 673, are quite remarkable. By taking any two primes and concatenating
 * them in any order the result will always be prime. For example, taking 7 and 109, both 7109 and
 * 1097 are prime. The sum of these four primes, 792, represents the lowest sum for a set of four
 * primes with this property.
 *
 * Find the lowest sum for a set of five primes for which any two primes concatenate to produce
 * another prime. */

extern crate shared;

use shared::digits;
use shared::primes;
use shared::sieve::{self, Sieve};

use std::cell::{RefCell, RefMut};
use std::collections::{BTreeMap, BTreeSet};

type Prime = u32;
type PrimeSets = BTreeMap<Prime, RefCell<BTreeSet<Prime>>>;

const SET_SIZE: usize = 5;
const SEGMENT_SIZE: usize = 200;

fn main() {
    let mut sieve: Sieve<Prime> = sieve::new();
    let mut sets: PrimeSets = BTreeMap::new();

    for (first_index, last_index) in (0..)
        .step_by(SEGMENT_SIZE)
        .zip((SEGMENT_SIZE..).step_by(SEGMENT_SIZE))
    {
        for prime in sieve.by_ref().take(SEGMENT_SIZE) {
            sets.insert(prime, RefCell::new(BTreeSet::new()));
        }

        for &prime in &sieve.found_primes()[first_index..last_index] {
            let mut prime_set: RefMut<BTreeSet<Prime>> = sets[&prime].borrow_mut();

            for (&other_prime, other_set_ref) in sets.iter() {
                if other_prime >= prime {
                    break;
                }
                if !concats_generate_primes(prime, other_prime) {
                    continue;
                }

                let mut other_set: RefMut<BTreeSet<Prime>> = other_set_ref.borrow_mut();

                other_set.insert(prime);
                prime_set.insert(other_prime);
            }
        }

        for &prime in sets.keys() {
            if let Some(result) = search_set(&[], prime, &sets) {
                println!("{}", result.into_iter().sum::<u32>());
                return;
            }
        }
    }
}

fn search_set(prev: &[Prime], prime: Prime, sets: &PrimeSets) -> Option<Vec<Prime>> {
    let concats_with_all_stack = prev.iter()
        .all(|prev_prime| sets[prev_prime].borrow().contains(&prime));

    if !concats_with_all_stack {
        return None;
    }

    let stack = {
        let mut stack = prev.to_vec();
        stack.push(prime);
        stack
    };

    if stack.len() == SET_SIZE {
        return Some(stack);
    }

    for &other_prime in sets[&prime].borrow().iter() {
        match search_set(&stack[..], other_prime, sets) {
            Some(v) => return Some(v),
            None => continue,
        }
    }

    None
}

fn concats_generate_primes(p1: Prime, p2: Prime) -> bool {
    return primes::is_prime(concat(p1, p2)) && primes::is_prime(concat(p2, p1));

    fn concat(start: Prime, end: Prime) -> Prime {
        let end_len = digits::new::<Prime, u8>(end).count();
        return start * 10u32.pow(end_len) + end;
    }
}
