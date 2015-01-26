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

#![allow(unstable)]
fn main() {
    let result = (2us..1_000).max_by(|&divisor| {
        match division_type(1, divisor) {
            DivisionType::Terminating      => 0,
            DivisionType::Recurring(cycle) => cycle.len()
        }
    });

    println!("{}", result.unwrap());
}

fn division_type(numerator: usize, denominator: usize) -> DivisionType {
    let mut seen: Vec<(usize, usize)> = Vec::new();
    let mut remaining = numerator;

    loop {
        while remaining < denominator {
            remaining *= 10;
            seen.push((remaining, 0));
        }

        let divided = remaining / denominator;
        remaining = remaining % denominator;

        if remaining == 0 {
            return DivisionType::Terminating;
        }

        remaining *= 10;

        let maybe_cycle = seen.iter().position(|&(past_remaining, _)| {
            past_remaining == remaining
        });

        match maybe_cycle {
            Some(start) => {
                return DivisionType::Recurring(seen_to_str(&seen[start..]));
            },

            None => {
                seen.push((remaining, divided));
            }
        }
    }
}

fn seen_to_str(vec: &[(usize, usize)]) -> String {
    let count = vec.len();

    let mut buffer = Vec::with_capacity(count);
    for &(_, divided) in vec.iter() {
        (write!(&mut buffer, "{}", divided)).unwrap();
    }

    String::from_utf8(buffer).unwrap()
}

#[derive(Show)]
enum DivisionType {
    Terminating,
    Recurring(String)
}
