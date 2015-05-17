use num::integer::{Integer, div_mod_floor};
use num::FromPrimitive; 

pub trait IntegerExtensions {
    fn is_palindrome(&self) -> bool;
    fn reverse(&self) -> Self;
}

impl<T: Integer + FromPrimitive + Copy> IntegerExtensions for T {
    fn is_palindrome(&self) -> bool {
        *self == self.reverse()
    }

    fn reverse(&self) -> T {
        let zero = FromPrimitive::from_u8(0).unwrap();
        let ten  = FromPrimitive::from_u8(10).unwrap();

        let mut remaining = *self;
        let mut reverse  = zero;

        while remaining > zero {
            let (next_remaining, current) = div_mod_floor(remaining, ten);
            reverse = (reverse * ten) + current;
            remaining = next_remaining;
        }

        reverse
    }
}
