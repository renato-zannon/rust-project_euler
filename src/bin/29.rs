#![feature(core)]

use std::collections::HashSet;
use std::num::Float;

fn main() {
    let mut powers: HashSet<(usize, usize)> = HashSet::new();

    for a in (2us..101) {
        let (base_a, factor) = factor_for(a);

        for b in (2us..101) {
            powers.insert((base_a, factor * b));
        }
    }

    println!("{}", powers.len());
}

fn factor_for(num: usize) -> (usize, usize) {
    let fnum:       f64 = num as f64;

    let mut factor: f64  = 1.0;
    let mut base:   usize = num;

    for potential_root in (2..num) {
        let exp = fnum.log(potential_root as f64);

        if exp.fract() == 0f64 {
            factor = exp;
            base   = potential_root;
            break;
        }
    }

    (base, factor as usize)
}
