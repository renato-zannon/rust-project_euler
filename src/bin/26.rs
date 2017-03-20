/* A unit fraction contains 1 in the numerator. The decimal representation of the unit fractions
 * with denominators 2 to 10 are given:
 *
 *     1/2	= 	0.5
 *     1/3	= 	0.(3)
 *     1/4	= 	0.25
 *     1/5	= 	0.2
 *     1/6	= 	0.1(6)
 *     1/7	= 	0.(142857)
 *     1/8	= 	0.125
 *     1/9	= 	0.(1)
 *     1/10	= 	0.1
 *
 * Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be seen that 1/7 has a
 * 6-digit recurring cycle.
 *
 * Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal
 * fraction part. */

use std::collections::HashMap;

fn main() {
    let mut seen: HashMap<u32, usize> = HashMap::new();

    let result = (2u32..1_000).max_by_key(|&divisor| {
        match division_type(1, divisor, &mut seen) {
            DivisionType::Terminating        => 0,
            DivisionType::Recurring { size } => size,
        }
    });

    println!("{}", result.unwrap());
}

fn division_type(numerator: u32, denominator: u32, seen: &mut HashMap<u32, usize>) -> DivisionType {
    seen.clear();
    let mut remaining = numerator;

    loop {
        while remaining < denominator {
            remaining *= 10;
            let len = seen.len();
            seen.insert(remaining, len);
        }

        let rest = remaining % denominator;
        if rest == 0 {
            return DivisionType::Terminating;
        }

        remaining = rest * 10;

        let maybe_cycle_start = seen.get(&remaining).cloned();

        let len = seen.len();
        seen.insert(remaining, len);

        match maybe_cycle_start {
            Some(start) => {
                return DivisionType::Recurring { size: (seen.len() - start - 1) as u16 };
            },

            None => (),
        }
    }
}

#[derive(Debug)]
enum DivisionType {
    Terminating,
    Recurring { size: u16 }
}
