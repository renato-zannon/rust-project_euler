use num::Integer;
use std::num::{Float, pow};
use std::fmt::Show;
use std::io::IoResult;

pub struct Digits<A, B> {
    remaining: A,
    remaining_digits: uint,
}

impl<A, B> Iterator<B> for Digits<A, B>
    where A: Integer + FromPrimitive + ToPrimitive, B: FromPrimitive {
    fn next(&mut self) -> Option<B> {
        if self.remaining_digits == 0 {
            return None;
        }

        let divisor: A = FromPrimitive::from_uint(self.current_divisor()).unwrap();
        let (digit, remainder) = self.remaining.div_rem(&divisor);

        self.remaining = remainder;
        self.remaining_digits -= 1;

        digit.to_u8().and_then(FromPrimitive::from_u8)
    }

    fn size_hint(&self) -> (uint, Option<uint>) {
        match self.remaining.to_f64() {
            Some(as_float) => {
                let log = as_float.log10();
                (log.floor() as uint, Some(log.ceil() as uint))
            },

            None => (0, None)
        }
    }

    fn count(&mut self) -> uint {
        let prev_count = self.remaining_digits;

        self.remaining_digits = 0;
        self.remaining = FromPrimitive::from_uint(0u).unwrap();

        prev_count
    }
}

impl<A, B> DoubleEndedIterator<B> for Digits<A, B>
    where A: Integer + FromPrimitive + ToPrimitive, B: FromPrimitive {
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
    where A: ToPrimitive + Show {
    Digits {
        remaining_digits: number_of_digits(&number),
        remaining: number,
    }
}

impl<A, B> Digits<A, B>
    where A: Integer + FromPrimitive + ToPrimitive {

    fn current_divisor(&self) -> uint {
        pow(10u, self.remaining_digits - 1)
    }
}

fn number_of_digits<A: ToPrimitive + Show>(number: &A) -> uint {
    let mut counter = DigitCounter { count: 0 };
    (write!(&mut counter, "{}", number)).unwrap();

    counter.count
}

struct DigitCounter {
    count: uint,
}

impl Writer for DigitCounter {
    fn write(&mut self, buf: &[u8]) -> IoResult<()> {
        self.count += buf.len();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::new;

    #[test]
    fn test_digits_in_order() {
        let digits = new(12345u).collect::<Vec<uint>>();
        assert_eq!(digits[], [1, 2, 3, 4, 5][]);
    }

    #[test]
    fn test_digits_in_reverse() {
        let digits = new(12345u).rev().collect::<Vec<uint>>();
        assert_eq!(digits[], [5, 4, 3, 2, 1][]);
    }

    #[test]
    fn test_digits_in_order_with_zero() {
        let digits = new(123450u).collect::<Vec<uint>>();
        assert_eq!(digits[], [1, 2, 3, 4, 5, 0][]);
    }

    #[test]
    fn test_digits_in_reverse_with_zero() {
        let digits = new(123450u).rev().collect::<Vec<uint>>();
        assert_eq!(digits[], [0, 5, 4, 3, 2, 1][]);
    }
}
