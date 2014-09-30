use std::iter::{range_step_inclusive, Unfold};

// Adapted from the problem 07 overview PDF
pub fn is_prime(num: uint) -> bool {
    if num == 1           { return false }
    else if num < 4       { return true  }
    else if num % 2 == 0  { return false }
    else if num < 9       { return true  }
    else if num % 3 == 0  { return false }

    let r = num.to_f64()
        .map(|as_float| as_float.sqrt())
        .and_then(|result| result.ceil().to_uint())
        .unwrap();

    range_step_inclusive(5, r, 6).all(|f| {
        num % f != 0 && num % (f + 2) != 0
    })
}

pub type PrimeFactors = Unfold<'static, uint, (uint, uint)>;

pub fn prime_factors(n: uint) -> PrimeFactors {
    return Unfold::new((n, 2), unfold_factors);

    fn unfold_factors(state_ptr : &mut (uint, uint)) -> Option<uint> {
        let (remaining, divisor) = *state_ptr;

        if remaining <= 1 {
            return None;
        }

        let mut new_divisor = divisor;
        while remaining % new_divisor > 0 {
            new_divisor += 1;
        }

        *state_ptr = (remaining / new_divisor, new_divisor);
        Some(new_divisor)
    }
}

pub type DistinctPrimeFactors = UniqueFilter<PrimeFactors>;

pub fn distinct_prime_factors(n: uint) -> DistinctPrimeFactors {
    UniqueFilter {
        last: None,
        iter: prime_factors(n)
    }
}

struct UniqueFilter<T> {
    last: Option<uint>,
    iter: T
}

impl<T: Iterator<uint>> Iterator<uint> for UniqueFilter<T> {
    fn next(&mut self) -> Option<uint> {
        loop {
            let new_value = match self.iter.next() {
                None        => return None,
                Some(value) => value,
            };

            match self.last {
                Some(old) if old == new_value => (),

                _ => {
                    self.last = Some(new_value);
                    return Some(new_value);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{distinct_prime_factors, prime_factors};

    macro_rules! prime_assert(
        ($func:ident, $num:expr, [ $($factor:expr),+ ]) => ({
            let mut expected_factors: Vec<uint> = vec!( $($factor),* );
            expected_factors.sort();

            let mut returned = $func($num).collect::<Vec<uint>>();
            returned.sort();

            assert_eq!(expected_factors, returned);
        });
    )

    #[test]
    fn test_prime_factors() {
        prime_assert!(prime_factors, 14, [2, 7]);
        prime_assert!(prime_factors, 15, [3, 5]);

        prime_assert!(prime_factors, 644, [2, 2, 7, 23]);
        prime_assert!(prime_factors, 645, [3, 5, 43]);
        prime_assert!(prime_factors, 646, [2, 17, 19]);
    }

    #[test]
    fn test_distinct_prime_factors() {
        prime_assert!(distinct_prime_factors, 2, [2]);
        prime_assert!(distinct_prime_factors, 4, [2]);
        prime_assert!(distinct_prime_factors, 8, [2]);
        prime_assert!(distinct_prime_factors, 27, [3]);

        prime_assert!(distinct_prime_factors, 644, [2, 7, 23]);
    }
}
