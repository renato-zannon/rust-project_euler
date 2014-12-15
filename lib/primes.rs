use std::iter::range_step_inclusive;
use std::num::{Float, Int};

// Adapted from the problem 07 overview PDF
pub fn is_prime<T>(num: T) -> bool
    where T : Int + Eq + Ord + FromPrimitive + Clone + ToPrimitive {

    let zero:  T = literal(0);
    let one:   T = literal(1);
    let two:   T = literal(2);
    let three: T = literal(3);
    let four:  T = literal(4);
    let five:  T = literal(5);
    let six:   T = literal(6);
    let nine:  T = literal(9);

    if num == one               { return false }
    else if num < four          { return true  }
    else if num % two == zero   { return false }
    else if num < nine          { return true  }
    else if num % three == zero { return false }

    let r = num.to_f32()
        .map(|as_float| as_float.sqrt())
        .map(|result| result.ceil())
        .and_then(|result| FromPrimitive::from_f32(result))
        .unwrap();

    return range_step_inclusive(five, r, six).all(|f| {
        num % f != zero && num % (f + two) != zero
    });

    #[inline]
    fn literal<T: FromPrimitive>(num: u8) -> T {
        FromPrimitive::from_u8(num).unwrap()
    }
}

pub struct PrimeFactors {
    remaining: uint,
    divisor: uint,
}

impl Iterator<uint> for PrimeFactors {
    fn next(&mut self) -> Option<uint> {
        if self.remaining <= 1 {
            return None;
        }

        let mut new_divisor = self.divisor;
        while self.remaining % new_divisor > 0 {
            new_divisor += 1;
        }

        self.remaining = self.remaining / new_divisor;
        self.divisor   = new_divisor;

        Some(new_divisor)
    }
}

pub fn prime_factors(n: uint) -> PrimeFactors {
    return PrimeFactors { remaining: n, divisor: 2 }
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
