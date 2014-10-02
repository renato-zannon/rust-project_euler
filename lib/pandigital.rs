use super::digits;
use super::std::{slice, iter};

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
            _            => false,
        }
    }
}

pub trait DigitCollection<T: Iterator<uint>> {
    fn digit_iter(self)     -> T;
    fn digit_len(&mut self) -> uint;
}

impl DigitCollection<digits::Digits<uint>> for digits::Digits<uint> {
    fn digit_iter(self) -> digits::Digits<uint> {
        return self
    }

    fn digit_len(&mut self) -> uint {
        9
    }
}

pub type SliceDigits<'a> = iter::Map<'a, &'a uint, uint, slice::Items<'a, uint>>;

impl<'a> DigitCollection<SliceDigits<'a>> for &'a [uint] {
    fn digit_iter(self) -> SliceDigits<'a> {
        return self.iter().map(|&digit| digit)
    }

    fn digit_len(&mut self) -> uint {
        self.len()
    }
}

pub fn is_9_pandigital<U: Iterator<uint>, T: DigitCollection<U>>(mut digits: T) -> PandigitalResult {
    match digits.digit_len().cmp(&9) {
        Less    => return TooSmall,
        Greater => return TooLarge,
        Equal   => (),
    }

    let mut found_numbers = [false, ..9];

    let only_uniques = digits.digit_iter().all(|digit| {
        let found = match digit {
            0     => return false,
            1...9 => &mut found_numbers[digit - 1],
            _     => unreachable!(),
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
        assert!(is_9_pandigital([1u, 2, 3, 4, 5, 6, 7, 8, 9][]).to_bool());
    }

    #[test]
    fn test_out_of_order() {
        assert!(is_9_pandigital([1u, 3, 5, 9, 7, 2, 8, 4, 6][]).to_bool());
    }

    #[test]
    fn test_not_all_numbers() {
        assert!(is_9_pandigital([1u, 2, 3, 4, 5, 6, 7][]).to_bool() == false);
    }

    #[test]
    fn test_with_repetitions() {
        assert!(is_9_pandigital([1u, 2, 3, 4, 5, 6, 7, 8, 9, 1][]).to_bool() == false);
    }

    #[test]
    fn test_rejects_zeroes() {
        assert!(is_9_pandigital([1u, 3, 5, 9, 7, 0, 0, 2, 8, 0, 0, 0, 4, 6, 0][]).to_bool() == false);
    }
}
