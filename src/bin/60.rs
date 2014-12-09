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

use shared::sieve::{mod, Sieve};
use shared::primes;
use shared::digits;

use std::collections::{TreeMap, TreeSet};
use std::cell::{RefCell, RefMut};
use std::iter::{count, AdditiveIterator};

type Prime = u32;
type PrimeSets = TreeMap<Prime, RefCell<TreeSet<Prime>>>;

const SET_SIZE: uint     = 5;
const SEGMENT_SIZE: uint = 200;

fn main() {
    let mut sieve: Sieve<Prime> = sieve::new();
    let mut sets:  PrimeSets = TreeMap::new();

    for (first_index, last_index) in count(0, SEGMENT_SIZE).zip(count(SEGMENT_SIZE, SEGMENT_SIZE)) {
        for prime in sieve.by_ref().take(SEGMENT_SIZE) {
            sets.insert(prime, RefCell::new(TreeSet::new()));
        }

        for &prime in sieve.found_primes().slice(first_index, last_index).iter() {
            let mut prime_set: RefMut<TreeSet<Prime>> = sets[prime].borrow_mut();

            for (&other_prime, other_set_ref) in sets.iter() {
                if other_prime >= prime { break }
                if !concats_generate_primes(prime, other_prime) { continue }

                let mut other_set: RefMut<TreeSet<Prime>> = other_set_ref.borrow_mut();

                other_set.insert(prime);
                prime_set.insert(other_prime);
            }
        }

        for &prime in sets.keys() {
            if let Some(result) = search_set(&[], prime, &sets) {
                println!("{}", result.into_iter().sum());
                return;
            }
        }
    }
}

fn search_set(prev: &[Prime], prime: Prime, sets: &PrimeSets) -> Option<Vec<Prime>> {
    let concats_with_all_stack = prev.iter().all(|&prev_prime| {
        sets[prev_prime].borrow().contains(&prime)
    });

    if !concats_with_all_stack { return None; }

    let stack = {
        let mut stack = prev.to_vec();
        stack.push(prime);
        stack
    };

    if stack.len() == SET_SIZE {
        return Some(stack)
    }

    for &other_prime in sets[prime].borrow().iter() {
        match search_set(stack.as_slice(), other_prime, sets) {
            Some(v) => return Some(v),
            None    => continue,
        }
    }

    None
}

fn concats_generate_primes(p1: Prime, p2: Prime) -> bool {
    use std::num::Int;

    return primes::is_prime(concat(p1, p2)) && primes::is_prime(concat(p2, p1));

    fn concat(start: Prime, end: Prime) -> Prime {
        let end_len = digits::new::<Prime, u8>(end).count();
        return start * 10.pow(end_len) + end;
    }
}
