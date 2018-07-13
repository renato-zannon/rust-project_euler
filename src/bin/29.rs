use std::collections::HashSet;

fn main() {
    let mut powers: HashSet<(u32, u32)> = HashSet::new();

    for a in 2..101 {
        let (base_a, factor) = factor_for(a);

        for b in 2..101 {
            powers.insert((base_a, factor * b));
        }
    }

    println!("{}", powers.len());
}

fn factor_for(num: u32) -> (u32, u32) {
    let fnum: f64 = num as f64;

    let mut factor: f64 = 1.0;
    let mut base: u32 = num;

    for potential_root in 2..num {
        let exp = fnum.log(potential_root as f64);

        if exp.fract() == 0f64 {
            factor = exp;
            base = potential_root;
            break;
        }
    }

    (base, factor as u32)
}
