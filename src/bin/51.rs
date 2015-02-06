/* Problem 51: Prime digit replacements
 *
 * By replacing the 1st digit of the 2-digit number *3, it turns out that six of the nine possible
 * values: 13, 23, 43, 53, 73, and 83, are all prime.
 *
 * By replacing the 3rd and 4th digits of 56**3 with the same digit, this 5-digit number is the
 * first example having seven primes among the ten generated numbers, yielding the family: 56003,
 * 56113, 56333, 56443, 56663, 56773, and 56993. Consequently 56003, being the first member of this
 * family, is the smallest prime with this property.
 *
 * Find the smallest prime which, by replacing part of the number (not necessarily adjacent digits)
 * with the same digit, is part of an eight prime value family. */

#![feature(core)]
extern crate shared;
use shared::{digits, sieve};

fn main() {
    let mut sieve = sieve::new();

    loop {
        let prime = sieve.next().unwrap();
        let prime_digits = digits::new(prime).collect::<Vec<usize>>();

        let families = (1..prime_digits.len()).flat_map(|variable_count| {
            families_from_variables(variable_count, &prime_digits).into_iter()
        });

        for family_iter in families {
            let family = family_iter.filter(|&num| sieve.is_prime(num)).collect::<Vec<usize>>();

            if family.len() == 8 {
                println!("{}", prime);
                return;
            }
        }
    }
}

fn families_from_variables(count: usize, digits: &Vec<usize>) -> Vec<FamilyIterator> {
    let mut result:    Vec<FamilyIterator> = Vec::new();
    let mut variables: Vec<usize>          = (0..count).map(|index| index).collect();

    let digit_count = digits.len();

    loop {
        result.push(FamilyIterator {
            variables: variables.clone(),
            template:  digits.clone(),
            last_used: None,
        });

        let mut max_index  = digit_count - 1;
        let mut current_index = variables.len();

        let var_slice = &mut variables[];

        loop {
            current_index = current_index - 1;

            let current = match var_slice.get(current_index) {
                Some(value) => value.clone(),
                None        => return result,
            };

            let next_value = current + 1;

            if next_value > max_index {
                var_slice[current_index] = match var_slice.get(current_index - 1) {
                    Some(&prev_var) => prev_var + 2,
                    None            => return result,
                };

                max_index -= 1;
            } else {
                var_slice[current_index] = next_value;
                break;
            }
        }
    }
}

struct FamilyIterator {
    variables: Vec<usize>,
    template:  Vec<usize>,
    last_used: Option<usize>,
}

impl Iterator for FamilyIterator {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let last_used = match self.last_used {
            val @ Some(_) => val,
            None          => return self.init_pattern(),
        };

        match last_used {
            None | Some(9) => None,

            Some(prev) => {
                let new   = prev + 1;
                let templ = &mut self.template[];

                for &var_index in self.variables.iter() {
                    templ[var_index] = new;
                }
                let new_value = FamilyIterator::to_number(templ);

                self.last_used = Some(new);
                Some(new_value)
            }
        }
    }
}

impl FamilyIterator {
    fn to_number(digits: &[usize]) -> usize {
        digits.iter().fold(0, |acc, &digit| {
            acc * 10 + digit
        })
    }

    fn init_pattern(&mut self) -> Option<usize> {
        let mut found_variable = None;
        let templ = &self.template[];

        for &var_index in self.variables.iter() {
            let on_template = match templ.get(var_index) {
                Some(&value) => value,
                None         => return None,
            };

            match found_variable {
                None       => found_variable = Some(on_template),
                Some(prev) => {
                    if prev != on_template {
                        return None;
                    }
                }
            }
        }

        self.last_used = found_variable;
        found_variable.map(|_| {
            FamilyIterator::to_number(&self.template[])
        })
    }
}
