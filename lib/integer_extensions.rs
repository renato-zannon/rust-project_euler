use num::integer::div_mod_floor;

pub trait IntegerExtensions {
    fn is_palindrome(&self) -> bool;
    fn reverse(&self) -> Self;
}

impl IntegerExtensions for uint {
    fn is_palindrome(&self) -> bool {
        *self == self.reverse()
    }

    fn reverse(&self) -> uint {
        let mut remaining = *self;
        let mut reverse   = 0;

        while remaining > 0 {
            let (next_remaining, current) = div_mod_floor(remaining, 10);
            reverse = (reverse * 10) + current;
            remaining = next_remaining;
        }

        reverse
    }
}
