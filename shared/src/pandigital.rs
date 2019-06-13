use self::PandigitalResult::{HasRepetitions, IsPandigital, TooLarge, TooSmall};
use super::digits;

use num::{FromPrimitive, Integer, ToPrimitive};
use std::{iter, slice};

#[derive(Copy, Clone)]
pub enum PandigitalResult {
    IsPandigital,
    TooSmall,
    TooLarge,
    HasRepetitions,
}

impl PandigitalResult {
    pub fn to_bool(&self) -> bool {
        match *self {
            IsPandigital => true,
            _ => false,
        }
    }
}

pub trait DigitCollection {
    type Iter: Iterator<Item = u8>;

    fn digit_iter(self) -> <Self as DigitCollection>::Iter;
    fn digit_len(&mut self) -> u32;
}

impl<A> DigitCollection for digits::Digits<A, u8>
where
    A: Integer + FromPrimitive + ToPrimitive,
{
    type Iter = digits::Digits<A, u8>;

    fn digit_iter(self) -> digits::Digits<A, u8> {
        return self;
    }

    fn digit_len(&mut self) -> u32 {
        9
    }
}

pub type SliceDigits<'a, N> = iter::Map<slice::Iter<'a, N>, fn(&'a N) -> u8>;

impl<'a, N> DigitCollection for &'a [N]
where
    N: ToPrimitive + Clone,
{
    type Iter = SliceDigits<'a, N>;

    fn digit_iter(self) -> SliceDigits<'a, N> {
        return self.iter().map(transform);

        fn transform<'a, N: ToPrimitive>(n: &'a N) -> u8 {
            ToPrimitive::to_u8(n).unwrap()
        }
    }

    fn digit_len(&mut self) -> u32 {
        self.len() as u32
    }
}

pub fn is_9_pandigital<T: DigitCollection>(mut digits: T) -> PandigitalResult {
    use std::cmp::Ordering::{Equal, Greater, Less};

    match digits.digit_len().cmp(&9) {
        Less => return TooSmall,
        Greater => return TooLarge,
        Equal => (),
    }

    let mut found_numbers = [false; 9];

    let only_uniques = digits.digit_iter().all(|digit| {
        let found = match digit {
            0 => return false,
            1..=9 => &mut found_numbers[(digit as usize) - 1],
            _ => unreachable!(),
        };

        if *found {
            return false;
        } else {
            *found = true;
            return true;
        }
    });

    if only_uniques {
        IsPandigital
    } else {
        HasRepetitions
    }
}

#[cfg(test)]
mod tests {
    use super::is_9_pandigital;

    #[test]
    fn test_1_through_9() {
        assert!(is_9_pandigital(&[1, 2, 3, 4, 5, 6, 7, 8, 9][..]).to_bool());
    }

    #[test]
    fn test_out_of_order() {
        assert!(is_9_pandigital(&[1, 3, 5, 9, 7, 2, 8, 4, 6][..]).to_bool());
    }

    #[test]
    fn test_not_all_numbers() {
        assert!(is_9_pandigital(&[1, 2, 3, 4, 5, 6, 7][..]).to_bool() == false);
    }

    #[test]
    fn test_with_repetitions() {
        assert!(is_9_pandigital(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 1][..]).to_bool() == false);
    }

    #[test]
    fn test_rejects_zeroes() {
        assert!(
            is_9_pandigital(&[1, 3, 5, 9, 7, 0, 0, 2, 8, 0, 0, 0, 4, 6, 0][..]).to_bool() == false
        );
    }
}
