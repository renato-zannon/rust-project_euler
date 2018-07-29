use num::{FromPrimitive, Integer, PrimInt, ToPrimitive};

use std::marker::PhantomData;

use digit_count::DigitCount;

pub struct Digits<A, B> {
    remaining: A,
    remaining_digits: u32,

    _marker: PhantomData<B>,
}

impl<A, B> Iterator for Digits<A, B>
where
    A: Integer + FromPrimitive + ToPrimitive,
    B: FromPrimitive,
{
    type Item = B;

    fn next(&mut self) -> Option<B> {
        if self.remaining_digits == 0 {
            return None;
        }

        let divisor: A = FromPrimitive::from_u32(self.current_divisor()).unwrap();
        let (digit, remainder) = self.remaining.div_rem(&divisor);

        self.remaining = remainder;
        self.remaining_digits -= 1;

        digit.to_u8().and_then(FromPrimitive::from_u8)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.remaining.to_f64() {
            Some(f) if f == 0.0 => (1, Some(1)),

            Some(as_float) => {
                let log = as_float.log10();
                (log.floor() as usize, Some(log.ceil() as usize))
            }

            None => (0, None),
        }
    }
}

impl<A, B> DoubleEndedIterator for Digits<A, B>
where
    A: Integer + FromPrimitive + ToPrimitive,
    B: FromPrimitive,
{
    fn next_back(&mut self) -> Option<B> {
        if self.remaining_digits == 0 {
            return None;
        }

        let ten: A = FromPrimitive::from_u8(10).unwrap();
        let (remainder, digit) = self.remaining.div_rem(&ten);

        self.remaining = remainder;
        self.remaining_digits -= 1;

        digit.to_u8().and_then(FromPrimitive::from_u8)
    }
}

pub fn new<A, B>(number: A) -> Digits<A, B>
where
    A: DigitCount,
{
    Digits {
        remaining_digits: number.number_of_digits(),
        remaining: number,
        _marker: PhantomData,
    }
}

impl<A, B> Digits<A, B>
where
    A: Integer + FromPrimitive + ToPrimitive,
{
    fn current_divisor(&self) -> u32 {
        10.pow(self.remaining_digits - 1)
    }

    pub fn count(self) -> u32 {
        self.remaining_digits
    }
}

#[cfg(test)]
mod tests {
    use super::new;

    #[test]
    fn test_digits_in_order() {
        let digits = new(12345usize).collect::<Vec<u32>>();
        assert_eq!(&digits, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_digits_in_reverse() {
        let digits = new(12345usize).rev().collect::<Vec<u32>>();
        assert_eq!(&digits, &[5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_digits_in_order_with_zero() {
        let digits = new(123450usize).collect::<Vec<u32>>();
        assert_eq!(&digits, &[1, 2, 3, 4, 5, 0]);
    }

    #[test]
    fn test_digits_in_reverse_with_zero() {
        let digits = new(123450usize).rev().collect::<Vec<u32>>();
        assert_eq!(&digits, &[0, 5, 4, 3, 2, 1]);
    }
}
