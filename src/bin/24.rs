/* Problem 24: Lexicographic permutations
 *
 * A permutation is an ordered arrangement of objects. For example, 3124 is one possible permutation
 * of the digits 1, 2, 3 and 4. If all of the permutations are listed numerically or alphabetically,
 * we call it lexicographic order. The lexicographic permutations of 0, 1 and 2 are:
 *
 * 012   021   102   120   201   210
 *
 * What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9? */

#![feature(slicing_syntax)]

fn main() {
    let result = permutations()
        .skip(1_000_000 - 1)
        .next();

    println!("{}", result);
}

fn permutations() -> SEPA<uint> {
    SEPA {
        current: vec!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9),
        first: true,
    }
}

// http://permute.tchs.info/soda_submit.php
struct SEPA<A> {
    current: Vec<A>,
    first: bool,
}

impl<A: Ord+Clone> Iterator<Vec<A>> for SEPA<A> {
    fn next(&mut self) -> Option<Vec<A>> {
        if self.first {
            self.first = false;
            return Some(self.current.clone());
        }

        self.keys().map(|(key, newkey)| {
            self.permute(key, newkey)
        })
    }
}

impl<A: Ord+Clone> SEPA<A> {
    fn keys(&self) -> Option<(uint, uint)> {
        let current_perm = self.current[];
        let current_len  = current_perm.len();

        let maybe_key_index: Option<uint> = range(1, current_len).rev().find(|&index| {
            let ref element = current_perm[index];
            let ref element_before = current_perm[index - 1];

            element > element_before
        }).map(|after_key_index| {
            after_key_index - 1
        });

        maybe_key_index.map(|key_index| {
            let ref key_element = current_perm[key_index];

            let newkey = range(key_index + 1, current_len).filter(|&index| {
                let ref element = current_perm[index];
                element > key_element
            }).min_by(|&index| {
                &current_perm[index]
            }).unwrap();

            (key_index, newkey)
        })
    }

    fn permute(&mut self, key: uint, newkey: uint) -> Vec<A> {
        let current_perm = self.current.as_mut_slice();

        current_perm.swap(key, newkey);

        let mut from_start = key + 1;
        let mut from_end   = current_perm.len() - 1;

        while from_end > from_start {
            current_perm.swap(from_end, from_start);
            from_end   -= 1;
            from_start += 1;
        }

        current_perm.into_vec()
    }
}
