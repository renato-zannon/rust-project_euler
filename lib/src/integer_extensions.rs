use num::integer::{Integer, div_mod_floor};
use num::FromPrimitive; 
use std::ops;

pub trait IntegerExtensions {
    fn is_palindrome(&self) -> bool;
    fn reverse(&self) -> Self;
}

impl<T: Integer + FromPrimitive + Clone> IntegerExtensions for T
    where for<'a> &'a T: ops::Mul<Output = T> + ops::Add<Output = T> {
    fn is_palindrome(&self) -> bool {
        *self == self.reverse()
    }

    fn reverse(&self) -> T {
        let zero: T = FromPrimitive::from_u8(0).unwrap();
        let ten:  T = FromPrimitive::from_u8(10).unwrap();

        let mut remaining = self.clone();
        let mut reverse   = zero.clone();

        while &remaining > &zero {
            let (next_remaining, current) = div_mod_floor(remaining, ten.clone());
            reverse = &((&reverse) * &ten) + &current;
            remaining = next_remaining;
        }

        reverse
    }
}
